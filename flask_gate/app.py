import requests
import random
from flask import Flask, jsonify

app = Flask("flask_gate")


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


@app.route('/<passenger>/<city>')
def recommend(passenger, city):
    past = []
    try:
        past = requests.get('http://192.168.32.3:5003/{}/{}'.format(passenger, city)).json()
    except:
        print("No past destinations for you, my friend!")
    return jsonify({"Addresses": address(past)})


import numpy as np
import math


class Integrator:
    def __init__(self, xMin, xMax, N):

    ################################

    def integrate(self):

    ##################################

    def show(self):


##################################


examp = Integrator(1, 3, 200)
examp.integrate()
examp.show()