import { ListItemNode, ListNode } from '@lexical/list';
import { css, html, LitElement } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';

import { $generateHtmlFromNodes, $generateNodesFromDOM } from '@lexical/html';
import { LinkNode } from '@lexical/link';
import { HeadingNode, QuoteNode } from '@lexical/rich-text';
import { $createParagraphNode, $createTextNode, $getRoot, createEditor, LexicalEditor, LineBreakNode, ParagraphNode, TextNode } from 'lexical';
import { createRef, ref, Ref } from 'lit/directives/ref.js';

@customElement('rich-text')
export class RichText extends LitElement {
  @property({ type: String })
  id!: string;



  @state()
  private error: string | null = null;

  @state()
  private loading: boolean = true;

  @state()
  private editing: boolean = false;



  @state() private _editor: LexicalEditor;
  @state() private canUndo = false;
  @state() private canRedo = false;

  contentEditableRef: Ref<HTMLDivElement> = createRef();

  constructor() {
    super();

    const config = {
      namespace: 'MyEditor',
      nodes: [
        HeadingNode,
        QuoteNode,
        ListItemNode,
        ListNode,
        LinkNode,
        ParagraphNode,
        TextNode,
        LineBreakNode,



      ],
      onError: (error: Error) => console.error(error),
    };

    this._editor = createEditor(config);
  }

  firstUpdated() {
    this._editor.setRootElement(this.contentEditableRef.value!);

    this._editor.update(() => {
      const root = $getRoot();
      if (root.getFirstChild() === null) {
        const paragraph = $createParagraphNode();
        paragraph.append($createTextNode(''));
        root.append(paragraph);
      }
    });

    this._editor.registerUpdateListener(({ editorState }) => {
      editorState.read(() => {
        // this.canUndo = this._editor.canUndo();
        // this.canRedo = this._editor.canRedo();
      });
    });

    this.fetchRichText();
  }


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

  // async updated(changedProperties: Map<string, any>) {
  //   if (changedProperties.has('id')) {
  //     const nodes = await this.fetchRichText();
  //     if(nodes){
  //       this.richTextNodes = nodes;
  //     }
  //   }
  // }

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
      const richText = data.rich_text;
      const parser = new DOMParser();
      const dom = parser.parseFromString(richText, 'text/html');
      const nodes = $generateNodesFromDOM(this._editor, dom);
      if (nodes) {
        this._editor.update(() => {
          const root = $getRoot();
          root.clear();
          root.append(...nodes);
        });
      }
      return nodes;
    } catch (e) {
      console.error('Failed to fetch rich text:', e);
      this.error = e instanceof Error ? e.message : 'An unknown error occurred.';
    } finally {
      this.loading = false;
    }
  }

  async postRichText() {
    try {
      const html = $generateHtmlFromNodes(this._editor);

      const response = await fetch(`http://127.0.0.1:3001/rich-text`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          id: this.id,
          rich_text: html
        }),
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      this.editing = false;
    } catch (e) {
      console.error('Failed to update rich text:', e);
      this.error = e instanceof Error ? e.message : 'An unknown error occurred while updating.';
    }
  }



  private async handleSave(e: Event) {
    e.preventDefault();
    e.stopPropagation();
    await this.postRichText();
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
        ${ref(this.contentEditableRef)}  
      ></div>
      <button class="edit-button" @click="${this.editing ? this.handleSave : this.handleEdit}">
        ${this.editing ? 'Save' : 'Edit'}
      </button>
      
    `;
  }
}
