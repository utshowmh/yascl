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

let arr = [1, 2, 3, 4, 5]
write(first(arr))
write(rest(arr))
