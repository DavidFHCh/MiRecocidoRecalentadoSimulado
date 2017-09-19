import sqlite3

conn = sqlite3.connect('world.db')

cities = [159, 31, 66, 221, 147, 200, 146, 1, 9, 84, 80, 29, 213, 158, 58, 122, 133, 264, 248, 23, 166, 70, 48, 150, 119, 176, 209, 101, 30, 39, 56, 117, 179, 65, 241, 178, 167, 94, 135, 190, 227, 86, 266, 232, 12, 203, 62, 90, 254, 5, 276, 160, 52, 75, 107, 92, 16, 233, 238, 244, 191, 207, 274, 215, 250, 22, 144, 188, 95, 186, 224, 216, 235, 185, 220, 143, 194, 198]

c = conn.cursor()

for id in cities:

    t = (id,)
    c.execute('SELECT latitude,longitude FROM cities WHERE id = ?',t)

# Do this instead
    coso = c.fetchone()
    print ("{},{}".format(coso[0], coso[1]))
