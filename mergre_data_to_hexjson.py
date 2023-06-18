import json
import csv

with open("Data/aberdeenshire_IZ.hexjson") as f:
    hexjson = json.load(f)

print(hexjson)
_ = [print(h) for h in hexjson["hexes"]]
    
with open("Data/IZdata.csv") as csvfile:
    dr = csv.reader(csvfile)
    rows = [r for r in dr]
    clients_male = {r[1]: {"client count": r[3],
                           "population": r[4],
                           "clients per 100 pop": r[5]}
                    for r in rows if r[2] == "Male"}
    clients_female = {r[1]: {"client count": r[3],
                           "population": r[4],
                           "clients per 100 pop": r[5]}
                    for r in rows if r[2] == "Female"}

#print(f"{len(clients_male)=} {clients_male= }")
# print(f"{clients_female= }")


male_hexes = {d["IZcode"]: d|clients_male[d["n"]] for h, d in hexjson["hexes"].items()}
# female_hexes = {d["IZcode"]: d|clients_female[d["n"]] for h, d in hexjson["hexes"].items()}

hexjson["hexes"] = male_hexes
with open("male_aberdeenshire_IZ.hexjson", "w") as outf:
    outf.write(json.dumps(hexjson))

# hexjson["hexes"] = female_hexes
# with open("female_aberdeenshire_IZ.hexjson", "w") as outf:
#     outf.write(jsom.dumps(hexjson))    
