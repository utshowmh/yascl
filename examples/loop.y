let for = |start, end, func| {
    if start != end {
        func(start)
        for(start + 1, end, func)
    }
}

let arr = array[1, 2, 3, 4, 5]

for(0, len(arr), |index| {
    write(arr[index])
})
