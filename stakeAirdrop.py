
import logging
from argparse import ArgumentParser
from pathlib import Path
from typing import cast

from erdpy.accounts import Account, Address
#from erdpy.accounts_repository import AccountsRepository
from erdpy.proxy import ElrondProxy
from erdpy.proxy.core import ElrondProxy
from erdpy.transactions import BunchOfTransactions, Transaction


def main():

    addressesToDistribute = [
        "erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd17c9u3mjanhht8fhkx0qyenka9xcu4dlu20h5d3ykrj7cpm0gk62qc6wy36",
"erd10z09avjakp5s0lx0azn5yaqhs3h2jr33tr9wa7qljt97npdv886qlahyhk",
"erd10z09avjakp5s0lx0azn5yaqhs3h2jr33tr9wa7qljt97npdv886qlahyhk",
"erd10z09avjakp5s0lx0azn5yaqhs3h2jr33tr9wa7qljt97npdv886qlahyhk",
"erd10z09avjakp5s0lx0azn5yaqhs3h2jr33tr9wa7qljt97npdv886qlahyhk",
"erd10z09avjakp5s0lx0azn5yaqhs3h2jr33tr9wa7qljt97npdv886qlahyhk",
"erd10z09avjakp5s0lx0azn5yaqhs3h2jr33tr9wa7qljt97npdv886qlahyhk",
"erd1tev8xfyj288cku6q2c4d32t458xznjycargw0azp06cd8clnjdvsu8syc9",
"erd1tev8xfyj288cku6q2c4d32t458xznjycargw0azp06cd8clnjdvsu8syc9",
"erd1tev8xfyj288cku6q2c4d32t458xznjycargw0azp06cd8clnjdvsu8syc9",
"erd1lrlykhdez72lycwj6clxmnqys89k6jwnx95s0zy36me55247686qtg2p88",
"erd1j00679yy98l7628jzvrpaaftvf53gzrfq2efft8rs0y62jwaf4rqt8wrcx",
"erd1hfwr8nad2svfnc94v9ap8z035uxxv2a5e96xrnurcah0asqgaldsecp6ga",
"erd1zhuy44gj5s6hynlznasne3jg6fd05xd62a3jp0re5aeaphpt3ewsnkzcru",
"erd1ssna3whcxze0kwh8fjlwrtyajf0fpprvlutu84mf762djn6j548qv4nczl",
"erd1ssna3whcxze0kwh8fjlwrtyajf0fpprvlutu84mf762djn6j548qv4nczl",
"erd1ssna3whcxze0kwh8fjlwrtyajf0fpprvlutu84mf762djn6j548qv4nczl",
"erd1fcff6lgvfkks2s2r2tc4z3lx4nhvqhxq6zs7y2ax5l47hjhvuj2qswrq83",
"erd1x5qndzw5l9g8agz9cx0svuy4h6qpjhn4d9l5a79szgxsra440vmq92wtrw",
"erd1lsxpxtam2fdxw0uvkkvkanka48z39kdt9ayq9m4849r9shzk53wsxwv5g2",
"erd1lsxpxtam2fdxw0uvkkvkanka48z39kdt9ayq9m4849r9shzk53wsxwv5g2",
"erd1lsxpxtam2fdxw0uvkkvkanka48z39kdt9ayq9m4849r9shzk53wsxwv5g2",
"erd1lsxpxtam2fdxw0uvkkvkanka48z39kdt9ayq9m4849r9shzk53wsxwv5g2",
"erd1lsxpxtam2fdxw0uvkkvkanka48z39kdt9ayq9m4849r9shzk53wsxwv5g2",
"erd1lsxpxtam2fdxw0uvkkvkanka48z39kdt9ayq9m4849r9shzk53wsxwv5g2",
"erd1mklay5n8cpye9jcukrgytnh95x5wkvx9ug00v8s9c54c5tnlzk8qd43uz0",
"erd1zw2wp83rcdyys5edsuh5ky5x8n639wwfhv6ued8n952aq7pagpfsxth365",
"erd1xgs477mlzxqw46wcdcqe8tg3dyzfh90kzkrgnpjf9h7vpsu59vpskcxed4",
    ]

    proxy = ElrondProxy("http://tortugaobs.defralcoding.it:8090")
    network = proxy.get_network_config()
    user = Account(pem_file="~/wallets/TiredApeClub.pem")
    user.sync_nonce(proxy)
    
    transactions: BunchOfTransactions = BunchOfTransactions()

    nftNonce = 610

    for addressToDistribute in addressesToDistribute:
        transaction = Transaction()
        transaction.nonce = user.nonce
        transaction.sender = user.address.bech32()
        transaction.receiver = user.address.bech32()
        transaction.value = "0"

        nonceEncoded = hex(nftNonce).replace("0x", "")
        if len(nonceEncoded) % 2 == 1:
            nonceEncoded = "0" + nonceEncoded

        data = "ESDTNFTTransfer"
        data += "@" + "5449524544434c55422d623863666238" # TIREDCLUB-b8cfb8
        data += "@" + nonceEncoded # nonce
        data += "@" + "01" # quantity
        data += "@" + "000000000000000005007499a1f0fadefe8675b4af7fbd28dd23e0ba4e89a938" # SC address      
        data += "@" + "61697264726f705365636f6e64" # airdropSecond
        data += "@" + Address(addressToDistribute).hex() # address real receiver

        transaction.data = data
        transaction.gasPrice = network.min_gas_price
        transaction.gasLimit = 6_000_000
        transaction.chainID = network.chain_id
        transaction.version = network.min_tx_version
        transaction.sign(user)

        transactions.add_prepared(transaction)
        
        #print(transaction.to_dictionary())
        
        user.nonce += 1
        nftNonce += 1
        
    #print(transactions)
    num, hashes = transactions.send(proxy)
    print(f"Sent {num} transactions")
    print("last nonce: " + str(nftNonce))
    #print(hashes)


if __name__ == "__main__":
    main()
