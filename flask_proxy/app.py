from flask import Flask, jsonify
from flask_sqlalchemy import SQLAlchemy


app = Flask("flask-proxy")
app.config['SQLALCHEMY_DATABASE_URI'] = 'postgresql://postgres:postgres@postgres:5432/pastdestinations'
db = SQLAlchemy(app)


class PastDestinations(db.Model):
    __tableanme__ = "pastdestinations"

    id = db.Column(db.Integer, primary_key=True, nullable=False, unique=True)
    passenger = db.Column(db.Integer, nullable=False)
    city = db.Column(db.String, nullable=False)
    street = db.Column(db.String)
    street_number = db.Column(db.String)
    times_visited = db.Column(db.Integer)

    def __init__(self, passenger, city, street, street_number, times_visited):
        self.passenger = passenger
        self.city = city
        self.street = street
        self.street_number = street_number
        self.times_visited = times_visited


db.create_all()


@app.route('/<passenger>/<city>')
def get_destinations(passenger, city):
    res = db.session.query(PastDestinations).filter(PastDestinations.passenger == passenger).filter(PastDestinations.city == city).all()
    response = {"Addresses": []}
    for addr in res:
        response['Addresses'].append({
            "Address": str(addr.street) + " " + str(addr.street_number),
            "count": str(addr["times_visited"])
        })
    return jsonify(res)
