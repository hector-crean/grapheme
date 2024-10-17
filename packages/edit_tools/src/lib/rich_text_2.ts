import { LitElement, html, css } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import { unsafeHTML } from 'lit/directives/unsafe-html.js';

@customElement('rich-text')
export class RichText extends LitElement {
  @property({ type: String })
  id!: string;

  @state()
  private richText: string = '';

  @state()
  private error: string | null = null;

  @state()
  private loading: boolean = true;

  @state()
  private editing: boolean = false;

  @state()
  private editableContent: string = '';

  static styles = css`
    :host {
      display: block;
      font-family: Arial, sans-serif;
      line-height: 1.6;
      color: #333;
      max-width: 800px;
      margin: 0 auto;
      padding: 20px;
    }
    .error {
      color: #ff0000;
      font-weight: bold;
    }
    .rich-text-content {
      background-color: #f9f9f9;
      border: 1px solid #e0e0e0;
      border-radius: 4px;
      padding: 15px;
    }
    .loading {
      text-align: center;
      font-style: italic;
    }
    .rich-text-content[contenteditable="true"] {
      border: 2px solid #007bff;
      outline: none;
    }
    .edit-button {
      margin-top: 10px;
      padding: 5px 10px;
      background-color: #007bff;
      color: white;
      border: none;
      border-radius: 4px;
      cursor: pointer;
    }
  `;

  updated(changedProperties: Map<string, any>) {
    if (changedProperties.has('id')) {
      this.fetchRichText();
    }
  }

  async fetchRichText() {
    this.loading = true;
    this.error = null;

    try {
      const response = await fetch(`http://127.0.0.1:3001/rich-text/${this.id}`);
      if (!response.ok) {
        throw new Error(response.status === 400
          ? 'Bad request. Please check the rich text ID.'
          : `HTTP error! status: ${response.status}`
        );
      }
      const data = await response.json();
      this.richText = data.rich_text;
      this.editableContent = this.richText; // Initialize editableContent
    } catch (e) {
      console.error('Failed to fetch rich text:', e);
      this.error = e instanceof Error ? e.message : 'An unknown error occurred.';
    } finally {
      this.loading = false;
    }
  }

  async postRichText(updatedText: string) {
    try {
      const response = await fetch(`http://127.0.0.1:3001/rich-text`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          id: this.id,
          rich_text: updatedText
        }),
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      this.richText = updatedText;
      this.editing = false;
    } catch (e) {
      console.error('Failed to update rich text:', e);
      this.error = e instanceof Error ? e.message : 'An unknown error occurred while updating.';
    }
  }

  private handleInput(e: Event) {
    const target = e.target as HTMLDivElement;
    this.editableContent = target.innerHTML;
  }

  private async handleSave(e: Event) {
    e.preventDefault();
    e.stopPropagation();
    await this.postRichText(this.editableContent);
    this.editing = false;
    this.requestUpdate();
  }

  private handleEdit() {
    this.editing = true;
  }

  render() {
    if (this.loading) {
      return html`<div class="loading">Loading...</div>`;
    }
    if (this.error) {
      return html`<div class="error">${this.error}</div>`;
    }
    return html`
      <div
        class="rich-text-content"
        contenteditable="${this.editing}"
        @input="${this.handleInput}"
      >
        ${unsafeHTML(this.editableContent)}
      </div>
      <button class="edit-button" @click="${this.editing ? this.handleSave : this.handleEdit}">
        ${this.editing ? 'Save' : 'Edit'}
      </button>
    `;
  }
}
