let selectedFaceDetector = 'tiny_face_detector'

let inputSize = 512

function onInputSizeChanged(e) {
  changeInputSize(e.target.value)
  updateResults()
}

function changeInputSize(size) {
  inputSize = parseInt(size)

  const inputSizeSelect = $('#inputSize')
  inputSizeSelect.val(inputSize)
  inputSizeSelect.formSelect()
}

function getCurrentFaceDetectionNet() {
    return faceapi.nets.tinyFaceDetector
}

function isFaceDetectionModelLoaded() {
  return !!getCurrentFaceDetectionNet().params
}

async function changeFaceDetector(detector) {
  selectedFaceDetector = detector
  const faceDetectorSelect = $('#selectFaceDetector')
  faceDetectorSelect.val(detector)
  faceDetectorSelect.formSelect()
 
  await getCurrentFaceDetectionNet().load('/weights')
}

async function onSelectedFaceDetectorChanged(e) {
  selectedFaceDetector = e.target.value

  await changeFaceDetector(e.target.value)
  updateResults()
}

function initFaceDetectionControls() {
  const faceDetectorSelect = $('#selectFaceDetector')
  faceDetectorSelect.val(selectedFaceDetector)
  faceDetectorSelect.on('change', onSelectedFaceDetectorChanged)
  faceDetectorSelect.formSelect()

  const inputSizeSelect = $('#inputSize')
  inputSizeSelect.val(inputSize)
  inputSizeSelect.on('change', onInputSizeChanged)
  inputSizeSelect.formSelect()
}

function resizeCanvasAndResults(dimensions, canvas, results) {
  const { width, height } = dimensions instanceof HTMLVideoElement
    ? faceapi.getMediaDimensions(dimensions)
    : dimensions
  canvas.width = width
  canvas.height = height

  // resize detections (and landmarks) in case displayed image is smaller than
  // original size
  return faceapi.resizeResults(results, { width, height })
}

function drawLandmarks(dimensions, canvas, results, withBoxes = true) {
  const resizedResults = resizeCanvasAndResults(dimensions, canvas, results)

  if (withBoxes) {
    faceapi.drawDetection(canvas, resizedResults.map(det => det.detection))
  }

  const faceLandmarks = resizedResults.map(det => det.landmarks)
  const drawLandmarksOptions = {
    lineWidth: 2,
    drawLines: true,
    color: 'green'
  }
  faceapi.drawLandmarks(canvas, faceLandmarks, drawLandmarksOptions)
}

async function onPlay() {
  const videoEl = document.getElementById('inputVideo')

  if(videoEl.paused || videoEl.ended || !isFaceDetectionModelLoaded())
    return setTimeout(() => onPlay())

  const options = new faceapi.TinyFaceDetectorOptions({inputSize: 512, scoreThreshold: 0.5})
  const result = await faceapi.detectSingleFace(videoEl, options).withFaceLandmarks()

  if (result) {
    drawLandmarks(videoEl, document.getElementById('overlay'), [result], true)
  }

  setTimeout(() => onPlay())
}

async function run() {
  await changeFaceDetector(selectedFaceDetector)
  await faceapi.loadFaceLandmarkModel('/weights')
  changeInputSize(224)

  const stream = await navigator.mediaDevices.getUserMedia({ video: {} })
  const videoEl = document.getElementById('inputVideo')
  videoEl.srcObject = stream
}

function updateResults() {}

function ready(fn) {
  if (document.attachEvent ? document.readyState === "complete" : document.readyState !== "loading"){
    fn();
  } else {
    document.addEventListener('DOMContentLoaded', fn);
  }
}     

ready(function() {
  initFaceDetectionControls()
  run()
})