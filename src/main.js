const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

window.addEventListener("DOMContentLoaded", () => {
    if (document.title === "Home") {
        loadStoryList();
        
        const add_btn = document.getElementById("add-story");
        add_btn.addEventListener("click", () => {
            navigateToPage("add-story");
        });
    }
    else if (document.title === "Add New Story") {
        const back_btn = document.getElementById("cancel-button");
        back_btn.addEventListener("click", () => {
            navigateToPage("index");
        });
    }
});

/**Load Project-List for homepage */
async function loadStoryList() {
  try {
      // Invoke the Rust command to get options
      const options = await invoke("get_story_list");
      
      // Get the select element
      const select = document.getElementById("story-list");

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

// Function to navigate to a page
// Listen for the "navigate" event
listen('navigate', (event) => {
    const pageName = event.payload;  // Get the page name from the event payload
    console.log(`Navigating to ${pageName}`);

    // Navigate to the new page
    window.location.href = `${pageName}.html`;
});

// Function to call from Rust to navigate to a new page
function navigateToPage(pageName) {
    invoke('navigate_to_page', { pageName })
        .then(() => {
            console.log(`Request sent to navigate to ${pageName}`);
        })
        .catch((err) => {
            console.error('Error navigating:', err);
        });
}