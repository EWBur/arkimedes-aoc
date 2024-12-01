def main():
    # Your main program logic goes here
    l=[]
    r=[]
    d={}
    with open("d1input.txt") as input:
        for line in input:
            line = line.strip().split()
            p1,p2 = map(int,line)
            l.append(p1)
            r.append(p2)
        l.sort()
        r.sort()

        #part one
        res1 = sum(abs(a-b) for a,b in zip(l,r))
        print(res1)

        #part two
        d = {num:r.count(num) for num in r}

        cost = 0
        for v in l:
            cost += d.get(v,0) *v 
        print(cost)


if __name__ == "__main__":
    main()