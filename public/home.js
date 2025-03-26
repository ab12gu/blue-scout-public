// Coral counter functionality
function incrementCoral(level) {
  const countElement = document.getElementById(`coral${level}Count`);
  const inputElement = document.getElementById(`coral${level}Input`);
  let currentValue = parseInt(countElement.value);

  // Check if we've hit the maximum value for L2, L3, L4
  if (level !== "L1" && currentValue >= 12) {
    return; // Don't increment if at max value
  }

  const newValue = currentValue + 1;
  countElement.value = newValue;
  inputElement.value = newValue;
}

function decrementCoral(level) {
  const countElement = document.getElementById(`coral${level}Count`);
  const inputElement = document.getElementById(`coral${level}Input`);
  let currentValue = parseInt(countElement.value);
  if (currentValue > 0) {
    const newValue = currentValue - 1;
    countElement.value = newValue;
    inputElement.value = newValue;
  }
}

function updateCoralInput(level) {
  const countElement = document.getElementById(`coral${level}Count`);
  const inputElement = document.getElementById(`coral${level}Input`);
  let value = parseInt(countElement.value);

  // Ensure value is a number
  if (isNaN(value)) {
    value = 0;
  }

  // Enforce min/max values
  if (value < 0) {
    value = 0;
  } else if (level !== "L1" && value > 12) {
    value = 12;
  }

  // Update both elements with validated value
  countElement.value = value;
  inputElement.value = value;
}

function resetCoralCounters() {
  ["L1", "L2", "L3", "L4"].forEach((level) => {
    document.getElementById(`coral${level}Count`).value = "0";
    document.getElementById(`coral${level}Input`).value = "0";
  });
  document.getElementById("coralDroppedCount").value = "0";
  document.getElementById("coralDroppedInput").value = "0";
}

// Dropped coral counter functionality
function incrementDroppedCoral() {
  const countElement = document.getElementById("coralDroppedCount");
  const inputElement = document.getElementById("coralDroppedInput");
  let currentValue = parseInt(countElement.value);

  const newValue = currentValue + 1;
  countElement.value = newValue;
  inputElement.value = newValue;
}

function decrementDroppedCoral() {
  const countElement = document.getElementById("coralDroppedCount");
  const inputElement = document.getElementById("coralDroppedInput");
  let currentValue = parseInt(countElement.value);
  if (currentValue > 0) {
    const newValue = currentValue - 1;
    countElement.value = newValue;
    inputElement.value = newValue;
  }
}

function updateDroppedCoralInput() {
  const countElement = document.getElementById("coralDroppedCount");
  const inputElement = document.getElementById("coralDroppedInput");
  let value = parseInt(countElement.value);

  // Ensure value is a number
  if (isNaN(value)) {
    value = 0;
  }

  // Enforce min value
  if (value < 0) {
    value = 0;
  }

  // Update both elements with validated value
  countElement.value = value;
  inputElement.value = value;
}

// Algae counter functionality
function incrementAlgae(location) {
  const countElement = document.getElementById(`algae${location}Count`);
  const inputElement = document.getElementById(`algae${location}Input`);
  let currentValue = parseInt(countElement.value);

  const newValue = currentValue + 1;
  countElement.value = newValue;
  inputElement.value = newValue;
}

function decrementAlgae(location) {
  const countElement = document.getElementById(`algae${location}Count`);
  const inputElement = document.getElementById(`algae${location}Input`);
  let currentValue = parseInt(countElement.value);
  if (currentValue > 0) {
    const newValue = currentValue - 1;
    countElement.value = newValue;
    inputElement.value = newValue;
  }
}

function updateAlgaeInput(location) {
  const countElement = document.getElementById(`algae${location}Count`);
  const inputElement = document.getElementById(`algae${location}Input`);
  let value = parseInt(countElement.value);

  // Ensure value is a number
  if (isNaN(value)) {
    value = 0;
  }

  // Enforce min value
  if (value < 0) {
    value = 0;
  }

  // Update both elements with validated value
  countElement.value = value;
  inputElement.value = value;
}

function resetAlgaeCounters() {
  ["FloorHole", "Barge"].forEach((location) => {
    document.getElementById(`algae${location}Count`).value = "0";
    document.getElementById(`algae${location}Input`).value = "0";
  });
}

// Update the form reset function to include all counters
document.addEventListener("DOMContentLoaded", function () {
  const form = document.querySelector("form");
  if (form) {
    // Add reset event handler (existing code)
    form.addEventListener("reset", function () {
      setTimeout(() => {
        resetCoralCounters();
        resetAlgaeCounters();
      }, 0);
    });

    form.addEventListener("submit", async function (event) {
      event.preventDefault();

      // Show loading state
      const submitBtn = form.querySelector('button[type="submit"]');
      const originalBtnText = submitBtn.innerHTML;
      submitBtn.disabled = true;
      submitBtn.innerHTML =
        '<span class="loading loading-spinner"></span> Submitting...';

      try {
        // Get form data
        const formData = new FormData(form);
        let formDataString = "";

        // Convert FormData to application/x-www-form-urlencoded format
        for (const [key, value] of formData.entries()) {
          formDataString +=
            encodeURIComponent(key) + "=" + encodeURIComponent(value) + "&";
        }

        // Remove the trailing '&'
        formDataString = formDataString.slice(0, -1);

        // Send data to server using fetch with application/x-www-form-urlencoded format
        const response = await fetch(form.action, {
          method: "POST",
          headers: {
            accept: "application/json",
            "content-type": "application/x-www-form-urlencoded",
          },
          body: formDataString,
        });

        if (!response.ok) {
          const errorText = await response.text();
          throw new Error(
            `HTTP error! Status: ${response.status}, Message: ${errorText}`,
          );
        }

        const result = await response.text(); // Assuming the response is in JSON

        if (result == "null") {
          // Reset the form
          form.reset();
          resetCoralCounters();
          resetAlgaeCounters();
        } else {
          throw new Error("Form submission returned an error status");
        }
      } catch (error) {
        console.error("Error submitting form:", error);
        alert(`Error submitting form: ${error.message}`);
      } finally {
        submitBtn.disabled = false;
        submitBtn.innerHTML = originalBtnText;
      }
    });
  }
});

function updateAutoInput(type) {
  const inputElement = document.getElementById(`auto${type}`);
  let value = parseInt(inputElement.value);

  // Ensure value is a number
  if (isNaN(value)) {
    value = 0;
  }

  if (value < 0) {
    value = 0;
  }

  inputElement.value = value;
}

function preventMinusSign(event) {
  if (event.key === "-") {
    event.preventDefault();
  }
}
