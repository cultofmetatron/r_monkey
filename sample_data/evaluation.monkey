let age = 1;
let name = "monkey";
let result = 10 * (20/2)

let myArray = [1, 2, 3, 4, 5];
let thorsten = {"name": "Thorsten", "age": 28};

let myArray[0] // 1
thorsten["name"] // => "Thorsten"

let add = fn(a, b) { return a + b; };

let add = fn(a, b) { a + b; };

let fibonacci = fn(x) { if (x == 0) {
    0 
  } else {
    if (x == 1) {
      1
    } else {
      fibonacci(x - 1) + fibonacci(x - 2);
    }
  }
};

