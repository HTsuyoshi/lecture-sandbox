import pandas as pd

# Open csv
df = pd.read_csv('pokemon_data.csv')

## Print first 3
# print(df.head(3))

## Print last 3
# print(df.tail(3))

## Print headers
# print(df.columns)

## Print Name column
# print(df['Name'])

## Print Name, HP and type 1
# print(df[['Name', 'Type 1', 'HP']])

## Read Each Row
# print(df.iloc[1])
# print(df.iloc[1:3])
# for index, row in df.iterrows():
#     print(index, row)

## Read a specific location
# print(df.iloc[1,2])

## Print all Fire types
# print(df.loc[df['Type 1'] == 'Fire'])

## Sort values by label
print(df.sort_values('Name', ascending=False))

## Sort values by labels
print(df.sort_values(['Name', 'HP'], ascending=False))



# df_xlsx = pd.read_excel('pokemon_data.xlsx', delim:ter='')










