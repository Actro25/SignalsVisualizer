const startButton = document.getElementById("ws_start");
const stopButton = document.getElementById("ws_stop");
const loadPlotButton = document.getElementById("load_plot");

const showPlotDiv = document.getElementById("show");

let ws = null;
let uplot = null;
let ringBuffer = null;

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
    setCapacity(capacity){
        this.capacity = capacity;
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

        return [outY, outX];
    }
}

startButton.addEventListener('click', function () {
    if(ws){
        console.log("Active.");
        return;
    }

    ws = new WebSocket("/ws");
    ws.onmessage = (event) => {
        try {
            const signal = JSON.parse(event.data);

            ringBuffer.push(signal.x,signal.y);

            uplot.setData(ringBuffer.getLinearData());
        }
        catch (err){
            console.log("Error JSON serializing: ", err);
        }
    }

    ws.onerror = (error) => {
        console.log("Error WebSocket: ", error);
    }
})

document.addEventListener("DOMContentLoaded", () => {
    ringBuffer = new RingBuffer(50);
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
    uplot = new uPlot(opts, [[],[]], showPlotDiv);
});

stopButton.addEventListener('click', function () {
    ws.close();
    ws = null;
})