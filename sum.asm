.loop 
    read        ; read in the next number
    bz print    ; if it's a zero, jump to printing
    add 99      ; if it's not a zero, add it to the sum
    sto 99      ; store the running sum
    b loop      ; go back to the beginning
.print
    load 99     ; place the sum in the calculator
    print       ; print the calculator display
    stop        ; stop the little man

