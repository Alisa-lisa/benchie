from sqlalchemy import Column, Integer, String
import sqlalchemy
import uvicorn
from starlette.middleware.database import DatabaseMiddleware
from starlette.applications import Starlette
from starlette.responses import JSONResponse


# Database table definitions.
pastdest = sqlalchemy.Table(
    'pastdestinations',  sqlalchemy.MetaData(),
    Column('id', Integer, primary_key=True),
    Column('passenger', Integer, nullable=False),
    Column('city', String, nullable=False),
    Column('street', String),
    Column('street_number', String),
    Column('times_visited', Integer)
)


app = Starlette()
app.add_middleware(DatabaseMiddleware,
                   database_url='postgresql://postgres:postgres@postgres:5432/pastdestinations',
                   rollback_on_shutdown=None)


@app.route('/{passenger}/{city}', methods=["GET"])
async def user(request):
    passenger = request.path_params['passenger']
    city = request.path_params['city']
    query = pastdest.select(pastdest.c.city == city).where(pastdest.c.passenger == int(passenger))
    results = await request.database.fetchall(query)
    content = [
        {
            "Address": str(result["street"]) + " " + str(result["street_number"]),
            "count": str(result["times_visited"])
        }
        for result in results
    ]
    return JSONResponse({"Addresses": content})


if __name__ == "__main__":
    # Only for debugging while developing
    uvicorn.run(app, host='0.0.0.0', port=5003, debug=True)
