width = 500
height = 500


for x in range(width):
    for y in range(height):


def scale_x(range,value):
    MAX_X = 0.47
    MIN_X = -2.00
    div = (MAX_X - MIN_X)/range
    return (MIN_X) + (value * div)
    

def scale_y(range,value):
    MAX_X = 1.12
    MIN_X = -2.00
    div = (MAX_X - MIN_X)/range
    return (MIN_X) + (value * div)
        
