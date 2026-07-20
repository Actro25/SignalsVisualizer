const connectButton = document.getElementById("ws_connect");
const disConnectButton = document.getElementById("ws_disconnect");
const pauseButton = document.getElementById("plot_pause");

const inputQuantityPoints = document.getElementById("points-range");
const spanQuantityPoints = document.getElementById("quantity-range-value");

const inputAmplitude = document.getElementById("amplitude-range");
const spanAmplitude = document.getElementById("amplitude-range-value");

const inputFrequency = document.getElementById("frequency-range");
const spanFrequency = document.getElementById("frequency-range-value");

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

    function getPlotSize() {
        return {
            width: showPlotDiv.clientWidth,
            height: showPlotDiv.clientHeight
        };
    }

    const opts = {
        ...getPlotSize(),
        series: [
            {},
            {
                label: "Value",
                stroke: "blue",
                width: 2,
            },
        ],
    };

    uplot = new uPlot(opts, [[], []], showPlotDiv);

    let resizeTimeout;
    window.addEventListener("resize", () => {
        cancelAnimationFrame(resizeTimeout);
        resizeTimeout = requestAnimationFrame(() => {
            if (uplot) {
                const size = getPlotSize();
                uplot.setSize(size);
            }
        });
    });
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


// === Quantity of points ===
inputQuantityPoints.addEventListener('input', function (event) {
    spanQuantityPoints.value = event.target.value;
});

inputQuantityPoints.addEventListener('change', function (event) {
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
    quantityOfPoints = numericValue;
    ringBuffer.setCapacity(quantityOfPoints);
});

spanQuantityPoints.addEventListener('change', function (event) {
    let val = Number(event.target.value);
    const min = Number(inputQuantityPoints.min);
    const max = Number(inputQuantityPoints.max);

    if (!isNaN(val)) {
        if (val < min) val = min;
        if (val > max) val = max;

        event.target.value = val;
        inputQuantityPoints.value = val;

        quantityOfPoints = val;
        ringBuffer.setCapacity(quantityOfPoints);
    }
});


// === Frequency ===
inputFrequency.addEventListener('input', function (event) {
    spanFrequency.value = event.target.value;
});

inputFrequency.addEventListener('change', function (event) {
    const changeEvent = new Event('change');
    spanFrequency.dispatchEvent(changeEvent);
});

spanFrequency.addEventListener('change', function (event) {
    let val = Number(event.target.value);
    const min = Number(inputFrequency.min);
    const max = Number(inputFrequency.max);

    if (!isNaN(val)) {
        if (val < min) val = min;
        if (val > max) val = max;

        event.target.value = val;
        inputFrequency.value = val;

        const command = {
            action: "change_frequency",
            value: val
        }
        ws.send(JSON.stringify(command));
    }
});


// === Amplitude ===
inputAmplitude.addEventListener('input', function (event) {
    spanAmplitude.value = event.target.value;
});

inputAmplitude.addEventListener('input', function (event) {
    const changeEvent = new Event('change');
    spanAmplitude.dispatchEvent(changeEvent);
});

spanAmplitude.addEventListener('change', function (event) {
    let val = Number(event.target.value);
    const min = Number(inputAmplitude.min);
    const max = Number(inputAmplitude.max);

    if (!isNaN(val)) {
        if (val < min) val = min;
        if (val > max) val = max;

        event.target.value = val;
        inputAmplitude.value = val;

        const command = {
            action: "change_amplitude",
            value: val
        }
        ws.send(JSON.stringify(command));
    }
});

disConnectButton.addEventListener('click', function () {
    ws.close();
    ws = null;
    ringBuffer = new RingBuffer(quantityOfPoints);
})