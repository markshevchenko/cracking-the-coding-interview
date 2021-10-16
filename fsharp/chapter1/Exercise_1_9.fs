module Exercise_1_9


let public isRotation (s1: string) (s2: string) =
    (s1 + s1).Contains(s2)