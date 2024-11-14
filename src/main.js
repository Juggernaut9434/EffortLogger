const { invoke } = window.__TAURI__.core;

window.addEventListener("DOMContentLoaded", () => {
  loadOptions();
});

/**Load Project-List for homepage */
async function loadOptions() {
  try {
      // Invoke the Rust command to get options
      const options = await window.__TAURI__.invoke("get_project_list");
      
      // Get the select element
      const select = document.getElementById("project-list");

      // Clear existing options
      select.innerHTML = '';

      // Populate the dropdown with the options
      options.forEach(option => {
          const opt = document.createElement("option");
          opt.value = option;
          opt.textContent = option;
          select.appendChild(opt);
      });
  } catch (error) {
      console.error("Failed to load options:", error);
  }
}