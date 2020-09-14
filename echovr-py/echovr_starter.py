#!/usr/bin/env python3
#
# Starter script for programmatic access of Echo VR API
# Copyright (C) 2020 Quentin Young
#
# Public domain, all rights released
#
# To run:
#    python echovr_starter.py
#
# If on Quest:
#    python echovr_starter.py --address <quest_wifi_ip_address>
#
# You can find your Quest IP address in the wifi settings on your Quest.
# Make sure to enable API Access in the settings in Echo VR.

import requests
import argparse
import time
import json
from requests.exceptions import ConnectionError
from json import JSONDecodeError


def get_frame(address):
    try:
        url = "http://{}:6721/session".format(address)
        print(url)
        response = requests.get(url, timeout=1)
    except ConnectionError as e:
        print("Connection error: {}".format(e))
        return None

    try:
        json_response = response.json()
    except JSONDecodeError as e:
        print("Response could not be decoded as JSON:\n{}".format(e))
        return None

    return json_response


parser = argparse.ArgumentParser()
parser.add_argument(
    "-a",
    "--address",
    required=False,
    type=str,
    help="IP address where Echo VR can be reached; if using Quest, this is your Quests's WiFi IP address",
    default="127.0.0.1",
)

args = parser.parse_args()

while True:
    json_response = get_frame(args.address)

    if not json_response:
        print("Could not access API, trying again in 3 seconds")
        time.sleep(3)
        continue

    print(json.dumps(json_response, indent=4, sort_keys=False))
