#The for loop construct in Reg-Lang allows the programmer to write code repeatedly for a fixed number of times like in any other languages, but with a new feature.

#For example a normal loop like in all the language:

#let x: int = 0;
for x to 5 iterate
    print x; # 0, 1, 2, 3, 4
end;
But you can do a decreasing loop like this:

#let x: int = 10;
for x to 5 iterate decreasing
    print x; # 10, 9, 8, 7, 6
end;
#You can also change the steps of the loop like this:

let x: int = 0;
for x to 5 step 1
    print x; # 0, 1, 2, 3, 4
end;
#Or you can also let the runtime choose wether it should decrease or increase the value of the loop:

let x: int = 0;
for x to 5 both 1 step
    print x; # 0, 1, 2, 3, 4
end;
let y: int = 10
for x to 5 both 1 step
    print y; # 10, 9, 8, 7, 6
end;