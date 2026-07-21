# SignalsVisualizer
This is a lightweight web application for viewing signals in graphical form.

***To use this web application, it's supposed that you've already has rust (1.96) and git on you pc.***

To use it, you'll need to clone my repository:
```shell
git clone https://github.com/Actro25/SignalsVisualizer.git
```

Next, go to the “SignalsVisualizer” folder.
```shell
cd SignalsVisualizer
```

Now you can simply run the application using the following command:
```shell
cargo run
```

# How to add you own signals?
It's simple. First of all you need to create you own structure that 
implements **generator** trait. Also make sure to add this Atomic fields: [ is_working, amplitude, frequency ].

Now you need to add your own structure into code. In file **index.rs** at **37** 
line you just need to change the following line:
```rust
let signal = Signals::new(working.clone(), amplitude.clone(), frequency.clone());
```
into this:
```rust
let signal = "Here is you structure name"::new(working.clone(), amplitude.clone(), frequency.clone());
```
