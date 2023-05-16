let for = |start, end, func| {
    if start != end {
        func(start)
        for(start + 1, end, func)
    }
}

let first = |arr| {
    if len(arr) > 0 {
        arr[0]
    }
}

let rest = |arr| {
    if len(arr) > 1 {
        arr[1..len(arr)]
    }
}

let map = |arr, func| {
    let acc = []
    for(0, len(arr), |index| {
        mut acc = append(acc, func(arr[index]))
    })
    acc
}

let arr = [1, 2, 3, 4, 5]
write(map(arr, |x| { x * 2 }))
