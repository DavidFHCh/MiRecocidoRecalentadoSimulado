import sqlite3

conn = sqlite3.connect('tsp.db')

cities = [216, 22, 717, 345, 447, 839, 829, 986, 505, 493, 935, 893, 521, 272, 492, 710, 572, 113, 642, 853, 137, 774, 1032, 299, 74, 627, 786, 180, 607, 1073, 109, 921, 679, 747, 200, 857, 498, 178, 117, 830]

         
c = conn.cursor()

for id in cities:

    t = (id,)
    c.execute('SELECT latitude,longitude FROM cities WHERE id = ?',t)

# Do this instead
    coso = c.fetchone()
    print ("{},{}".format(coso[0], coso[1]))
