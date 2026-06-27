const pairPanel = document.getElementById('pairPanel');
const pairedPanel = document.getElementById('pairedPanel');
const statusText = document.getElementById('statusText');
const message = document.getElementById('message');
const pinInput = document.getElementById('pinInput');
const pairButton = document.getElementById('pairButton');
const forgetButton = document.getElementById('forgetButton');
const openAssistantButton = document.getElementById('openAssistantButton');

function setMessage(text, isError = true) {
  message.textContent = text;
  message.style.color = isError ? '#9b2f2f' : '#24683d';
}

function render(paired) {
  pairPanel.hidden = paired;
  pairedPanel.hidden = !paired;
  pairPanel.style.display = paired ? 'none' : 'grid';
  pairedPanel.style.display = paired ? 'grid' : 'none';
  statusText.textContent = paired ? 'Paired and ready' : 'Not paired yet';
}

chrome.storage.local.get(['civicnews_token'], (result) => {
  render(Boolean(result.civicnews_token));
});

pairButton.addEventListener('click', () => {
  const pin = pinInput.value.trim();
  if (!pin) {
    setMessage('Paste the pairing code from The Civic Desk.');
    return;
  }

  pairButton.disabled = true;
  setMessage('Pairing...', false);
  chrome.runtime.sendMessage({ type: 'CN_PAIR', pin }, (response) => {
    pairButton.disabled = false;
    if (response && response.success) {
      pinInput.value = '';
      setMessage('Paired. Open ChatGPT, Claude, or Gemini to use the bridge.', false);
      render(true);
    } else {
      setMessage(response?.error || 'Pairing failed. Check that The Civic Desk is running.');
      render(false);
    }
  });
});

forgetButton.addEventListener('click', () => {
  chrome.storage.local.remove(['civicnews_token'], () => {
    setMessage('Pairing removed from this browser.', false);
    render(false);
  });
});

openAssistantButton.addEventListener('click', () => {
  chrome.tabs.create({ url: 'https://chatgpt.com/' });
});
