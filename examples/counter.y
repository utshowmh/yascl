let counter = || {
    let count = 0
    let _counter = || {
        mut count = count + 1
    }
    _counter
}

let count = counter()
write(count())
write(count())
write(count())

