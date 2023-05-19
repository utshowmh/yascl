let for = |start, end, func| {
    if start != end {
        func(start)
        for(start + 1, end, func)
    }
}

let map = |arr, func| {
    let acc = array[]
    for(0, len(arr), |index| {
        mut acc = append(acc, func(arr[index]))
    })
    acc
}

let arr = array[1, 2, 3, 4, 5]
write(map(arr, |x| { x * 2 }))
