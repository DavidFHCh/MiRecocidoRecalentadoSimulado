import sqlite3

conn = sqlite3.connect('world.db')

cities = [56, 30, 117, 179, 39, 241, 101, 65, 209, 94, 178, 176, 119, 135, 190, 999, 227, 167, 86, 150, 266, 107, 12, 191, 70, 75, 92, 16, 48, 233, 244, 207, 232, 248, 194, 264, 238, 254, 203, 52, 274, 90, 216, 62, 122, 23, 250, 143, 198, 5, 160, 166, 200, 66, 84, 133, 29, 186, 22, 215, 159, 31, 158, 221, 188, 213, 146, 147, 80, 224, 185, 235, 144, 95, 1, 9, 220, 58, 276]
          
c = conn.cursor()

for id in cities:

    t = (id,)
    c.execute('SELECT latitude,longitude FROM cities WHERE id = ?',t)

# Do this instead
    coso = c.fetchone()
    print ("{},{}".format(coso[0], coso[1]))
