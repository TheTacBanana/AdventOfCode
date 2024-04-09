(define (char_dir c)
    (if (eq? c #\()
        1
        (if (eq? c #\))
            -1
            0
        )
    )
)

(define (solve chars acc)
    (if (empty? chars)
        acc
        (let (
            (dir (char_dir (car chars)))
        )
            (solve (cdr chars) (+ acc dir))
        )
    )
)

(define (solve_p2 chars acc n)
    (if (eq? acc -1)
        n
        (solve_p2
            (cdr chars)
            (+ acc (char_dir (car chars)))
            (+ n 1)
        )
    )
)

(define brackets (string->chars (file->string "./examples/input.txt")))

(write (solve brackets 0))
(write (solve_p2 brackets 0 0))
