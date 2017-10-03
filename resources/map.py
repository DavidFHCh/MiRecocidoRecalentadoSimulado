import sqlite3

conn = sqlite3.connect('world.db')

cities = [19, 98, 769, 852, 819, 247, 671, 477, 319, 360, 22, 432, 587, 198, 980, 687, 742, 225, 304, 554, 250, 770, 406, 607, 334, 970, 1027, 337, 55, 652, 1089, 969, 368, 922, 303, 539, 505, 917, 312, 75, ]
         
c = conn.cursor()

for id in cities:

    t = (id,)
    c.execute('SELECT latitude,longitude FROM cities WHERE id = ?',t)

# Do this instead
    coso = c.fetchone()
    print ("{},{}".format(coso[0], coso[1]))
