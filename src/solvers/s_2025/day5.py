from sympy import S,Union, Interval,FiniteSet,Naturals0,Symbol
test = "0-1\n10-60\n50-60"
with open("/home/cydget/Code/RUST/AOC_2025/AoC_Rust/inputs/2025/5/input.txt") as f:
    test = f.read()


rows = test.split("\n")
rows = [row for row in rows if len(row)>2 and "-" in row]
#rows = rows[0:5]
#print(rows)
Intervals = [Interval(int(data[0]),int(data[1])) for data in [row.split("-") for row in rows ]]
End_interval = Intervals.pop()
while len(Intervals) >0:
    I_next = Intervals.pop()
    End_interval = Union(End_interval,I_next)
    print(f"We have{len(Intervals)} left")
print(End_interval)
Result_iter = iter([A for A in End_interval.args ])

Result = 0
for A in Result_iter:
    if type(A) is FiniteSet:
        print("set")
        Result+=len(A)
    else:
        r=(A.end-A.start+1) 
        Result+=r
print(f"Result is:{Result}")
#range-set     v0.1.0
#range-set-blaze   v0.4.1
