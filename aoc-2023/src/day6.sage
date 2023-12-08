# cheese solution in sagemath

def part1(t,d):
    var('x')
    b = solve([x*(t-x)>d],x)[0]
    lb = (b[0].rhs())
    eb = (b[1].rhs())
    lb_approx = floor(lb)
    eb_approx = floor(eb)
    if not (bool(b[0](x=lb_approx))):
        lb_approx = lb_approx + 1
    if not (bool(b[1](x=eb_approx))):
        eb_approx = eb_approx - 1
    result = eb_approx - lb_approx + 1
    return result
    
times = [48  ,   87   ,  69   ,  81]
distances = [255 ,  1288 ,  1117 ,  1623]
ba = 1
for i in range(len(times)):
    ba*=part1(times[i],distances[i])
    
print(ba)
print(part1(48876981,255128811171623))
