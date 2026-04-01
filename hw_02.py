from collections import defaultdict
import time
import pandas as pd
from zeep import Client

from zeep import Client

WSDL_URL = "http://andromeda.fi.muni.cz/~xbatko/homework2?wsdl"


client = Client(WSDL_URL)
email = "514244@mail.muni.cz"
# vary causiously picked numbers to diversify a portfolio
STOCKS_TO_BUY = 2

print("\n=== SERVICE OPERATIONS ===")
for service in client.wsdl.services.values():
    for port in service.ports.values():
        for op_name, op in port.binding._operations.items():
            print(f"  - {op_name}: {op}")
print()


token = client.service.createAccount(email)

print("\n=== BALANCE ===")
balance = client.service.balance(token)
print(f"Account balance: {balance}")


print("\n=== HOW MUCH TO FINISH ===")
to_finish = client.service.howMuchToFinish(token)
print(f"Missing gain: {to_finish}")

stocks = client.service.list()

print("\n=== STOCKS ===")
# collecting data about the market to make some smart decisions and become the next Warren Buffett
history = defaultdict(list)
for _ in range(40):
    for stock in stocks:
        current_stock_price = client.service.info(stock)
        history[stock].append(current_stock_price)
    time.sleep(0.5)

df = pd.DataFrame(history)
# apparently the stocks prices cycle, so only thing to be done,
# is to buy at the lowest price and sell at the highest - let's show those guyz on Wall Street how it is done
print(df.describe())

# here is the MAGIC
min_prices = {stock: int(df[stock].min()) for stock in stocks}
max_prices = {stock: int(df[stock].max()) for stock in stocks}

owned_stocks = defaultdict(int)

while True:
    for stock in stocks:
        current_stock_price = client.service.info(stock)
        try:
            if (
                current_stock_price < balance
                and current_stock_price <= min_prices[stock]
            ):
                print(f"Buying {STOCKS_TO_BUY}x{stock} for {current_stock_price}")
                client.service.buy(token, stock, STOCKS_TO_BUY)
                owned_stocks[stock] += STOCKS_TO_BUY
                balance = client.service.balance(token)
            elif owned_stocks[stock] > 0 and current_stock_price >= max_prices[stock]:
                print(
                    f"Selling {owned_stocks[stock]}x{stock} for {current_stock_price}"
                )
                client.service.sell(token, stock, owned_stocks[stock])
                owned_stocks[stock] = 0
                balance = client.service.balance(token)
        except Exception as e:
            print(f"Error: {str(e)}")
            pass
    print(f"Current balance: {balance}")
    to_finish = client.service.howMuchToFinish(token)
    print(f"Missing gain: {to_finish}")
    print(f"Owned:")
    for stock in stocks:
        print(
            f"    {stock}: {client.service.own(token, stock)} with current price: {client.service.info(stock)}"
        )
    print("#" * 5)

    if to_finish <= 0:
        break
    # sleep for 1 sec to not overflow fi servers - being nice here
    time.sleep(0.5)


print("Lets gooo")
