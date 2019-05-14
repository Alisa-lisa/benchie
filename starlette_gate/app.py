import random
import uvicorn
from starlette.applications import Starlette
from starlette.responses import JSONResponse
import aiohttp
import concurrent.futures as fut


app = Starlette()
session = aiohttp.ClientSession()
executor = fut.ThreadPoolExecutor(max_workers=4)


def get_random_address(suggestions, street_base):
    res = []
    for _ in range(suggestions):
        res.append("{} {}".format(street_base, random.randint(suggestions + 1, suggestions * 100)))
    return res


def get_best_addresses(addresses):
    random.shuffle(addresses)
    return addresses[:3]


def address(another_list=None):
    pois = get_random_address(5, "POI Street")
    fav = get_random_address(random.randint(0, 3), "Favorite Street")
    if another_list:
        return {"recommendation": get_best_addresses(pois + fav + another_list)}
    return {"recommendation": get_best_addresses(pois + fav)}


@app.route('/{passenger}/{city}', methods=["GET"])
async def recommend(request):
    passenger = request.path_params['passenger']
    city = request.path_params['city']
    past = {"Addresses":[]}
    try:
        async with aiohttp.ClientSession() as session:
            # async with session.get('http://172.20.0.3:5003/{}/{}'.format(passenger, city)) as resp:
            async with session.get('http://proxy:5003/{}/{}'.format(passenger, city)) as resp:
                past = await resp.json()
    except:
        pass
    return JSONResponse({"Addresses": address(past['Addresses'])})


if __name__ == "__main__":
    # Only for debugging while developing
    uvicorn.run(app, host='0.0.0.0', port=5002, debug=True)
