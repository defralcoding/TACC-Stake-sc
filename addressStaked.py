
import logging
from argparse import ArgumentParser
from pathlib import Path
from typing import cast
import json

from erdpy.accounts import Account, Address
from erdpy.contracts import CodeMetadata, SmartContract
from erdpy.proxy.core import ElrondProxy

def main():

    proxy = ElrondProxy("http://tortugaobs.defralcoding.it:8090")
    network = proxy.get_network_config()

    #Read json file usersStaked.json
    file = open("usersStaked.json", "r")
    usersStaked = json.load(file)

    contract_address = Address("erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h")
    contract = SmartContract(address=contract_address)

    function = "getUserStakedFirstCollection"
    

    for user in usersStaked:
        userObj = Address(user["hex"])

        answer = contract.query(proxy, function, ["0x" + userObj.hex()])
        for _ in answer:
            print("\"" + userObj.bech32() + "\"" + ",")

if __name__ == "__main__":
    main()
