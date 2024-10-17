// Function to get rich text by ID
const BASE_URL = 'http://localhost:3001';

async function getRichText(id: string): Promise<string> {
    const response = await fetch(`${BASE_URL}/rich-text/${id}`, {
      method: 'GET',
      headers: {
        'Accept': 'application/json',
      },
    });
  
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
  
    const data = await response.json();
    return data.content;
  }
  
  // Function to post rich text
  async function postRichText(id: string, content: string): Promise<string> {
    const response = await fetch('${BASE_URL}/rich-text', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
      body: JSON.stringify({ content }),
    });
  
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
  
    const data = await response.json();
    return data.id;
  }
  

  export { getRichText, postRichText };
  
