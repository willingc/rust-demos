fn main() {
    do 10.times {
        do spawn {
            let greeting_message = "Hello earthlings?";
            println(greeting_message);
        }
    }
}

