const connectButton = document.getElementById("ws_connect");
const disConnectButton = document.getElementById("ws_disconnect");
const pauseButton = document.getElementById("plot_pause");
const applyButton = document.getElementById("apply");
const inputQuantityPoints = document.getElementById("quantity-input");

const showPlotDiv = document.getElementById("show");

let ws = null;
let uplot = null;
let ringBuffer = null;
let isWork = true;
let quantityOfPoints = 1000;

class RingBuffer {
    constructor(capacity) {
        this.capacity = capacity;
        this.xs = new Float64Array(capacity);
        this.ys = new Float64Array(capacity);
        this.head = 0;
        this.tail = 0;
        this.isFull = false;
    }

    push(x, y) {
        this.xs[this.tail] = x;
        this.ys[this.tail] = y;

        if (this.isFull) {
            this.head = (this.head + 1) % this.capacity;
        }

        this.tail = (this.tail + 1) % this.capacity;

        if (this.tail === this.head) {
            this.isFull = true;
        }
    }

    setCapacity(capacity) {
        const [xs, ys] = this.getLinearData();

        this.capacity = capacity;
        this.xs = new Float64Array(capacity);
        this.ys = new Float64Array(capacity);
        this.head = 0;
        this.tail = 0;
        this.isFull = false;

        const n = Math.min(xs.length, capacity);
        for (let i = xs.length - n; i < xs.length; i++) {
            this.push(xs[i], ys[i]);
        }
    }

    getLinearData() {
        const size = this.isFull ? this.capacity : this.tail;
        const outX = new Float64Array(size);
        const outY = new Float64Array(size);

        if (!this.isFull) {
            outX.set(this.xs.subarray(0, this.tail));
            outY.set(this.ys.subarray(0, this.tail));
        } else {
            const rightSize = this.capacity - this.head;
            outX.set(this.xs.subarray(this.head, this.capacity), 0);
            outX.set(this.xs.subarray(0, this.head), rightSize);

            outY.set(this.ys.subarray(this.head, this.capacity), 0);
            outY.set(this.ys.subarray(0, this.head), rightSize);
        }

        return [outX, outY];
    }
}

connectButton.addEventListener('click', function () {
    if (ws) {
        console.log("Active.");
        return;
    }

    ws = new WebSocket("/ws");
    ws.onmessage = (event) => {
        try {
            const signal = JSON.parse(event.data);

            ringBuffer.push(signal.x, signal.y);

            if (isWork) {
                uplot.setData(ringBuffer.getLinearData());
            }
        } catch (err) {
            console.log("Error JSON serializing: ", err);
        }
    }

    ws.onerror = (error) => {
        console.log("Error WebSocket: ", error);
    }
})

document.addEventListener("DOMContentLoaded", () => {
    ringBuffer = new RingBuffer(1000);
    const opts = {
        width: 800,
        height: 400,
        series: [
            {},
            {
                label: "Значення",
                stroke: "blue",
                width: 2,
            },
        ],
    };
    uplot = new uPlot(opts, [[], []], showPlotDiv);
});

pauseButton.addEventListener('click', function () {
    isWork = !isWork;
    if(!isWork){
        pauseButton.textContent = "Continue";
    }
    else{
        pauseButton.textContent = "Pause";
    }

})

applyButton.addEventListener('click', function () {
    let inputValue = inputQuantityPoints.value;

    if (inputValue === '') {
        alert('Please, enter correct value!');
        return;
    }

    const numericValue = Number(inputValue);
    if (numericValue <= 0) {
        alert('Please, enter correct value!');
        return;
    }
    quantityOfPoints = numericValue
    ringBuffer.setCapacity(quantityOfPoints);
})

disConnectButton.addEventListener('click', function () {
    ws.close();
    ws = null;
    ringBuffer = new RingBuffer(quantityOfPoints);
})