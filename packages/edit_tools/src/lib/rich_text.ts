import { createEmptyHistoryState, registerHistory } from '@lexical/history';
import { $generateHtmlFromNodes, $generateNodesFromDOM } from '@lexical/html';
import { LinkNode, TOGGLE_LINK_COMMAND } from '@lexical/link';
import { INSERT_ORDERED_LIST_COMMAND, INSERT_UNORDERED_LIST_COMMAND, ListItemNode, ListNode, REMOVE_LIST_COMMAND } from '@lexical/list';
import { HeadingNode, QuoteNode, registerRichText } from '@lexical/rich-text';
import { $isAtNodeEnd } from "@lexical/selection";
import { mergeRegister } from '@lexical/utils';
import { $createParagraphNode, $createTextNode, $getRoot, $getSelection, $isElementNode, $isRangeSelection, CAN_REDO_COMMAND, CAN_UNDO_COMMAND, COMMAND_PRIORITY_EDITOR, COMMAND_PRIORITY_LOW, createEditor, ElementFormatType, FORMAT_ELEMENT_COMMAND, FORMAT_TEXT_COMMAND, LexicalEditor, LineBreakNode, ParagraphNode, RangeSelection, REDO_COMMAND, TextFormatType, TextNode, UNDO_COMMAND } from 'lexical';
import { css, html, LitElement } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import { createRef, ref, Ref } from 'lit/directives/ref.js';
import { styleMap } from 'lit/directives/style-map.js';
import { getRichText, postRichText } from './requests';

// Define a simple Result type
type Result<T, E = Error> = { ok: true; value: T } | { ok: false; error: E };



namespace RichTextRequest {
  // Wrap the API calls in Result-returning functions
  export async function post(id: string, content: string): Promise<Result<string>> {
    try {
      const newId = await postRichText(id, content);
      return { ok: true, value: newId };
    } catch (error) {
      return { ok: false, error: error instanceof Error ? error : new Error(String(error)) };
    }
  }

  export async function get(id: string): Promise<Result<string>> {
    try {
      const content = await getRichText(id);
      return { ok: true, value: content };
    } catch (error) {
      return { ok: false, error: error instanceof Error ? error : new Error(String(error)) };
    }
  }
}



export default function $prepopulatedRichText() {
  const root = $getRoot();
  if (root.getFirstChild() !== null) {
    return;
  }

  const paragraph = $createParagraphNode();
  paragraph.append(
    $createTextNode('[CLICK TO EDIT]'),
  );
  root.append(paragraph);
}



@customElement('rich-text')
export class RichText extends LitElement {
  @property({ type: Boolean, reflect: true }) editable = true;
  @property({ type: String, reflect: true }) id = "";
  @state() private _editor: LexicalEditor;
  @state() private canUndo = false;
  @state() private canRedo = false;
  @state() private toolbarPosition = { top: '0px', left: '0px' };
  @state() private showToolbar = false;


  contentEditableRef: Ref<HTMLDivElement> = createRef();

  static styles = css`
    :host {
      display: block;
      font-family: Arial, sans-serif;
    }

    .editor-wrapper {
      overflow: hidden;
      position: relative;
      cursor: text;
      display: flex;
      flex-direction: column;
    }


    [contenteditable] {
      outline: none;
    }

    [contenteditable]:focus {
      box-shadow: 0 0 0 2px #007bff;
    }

    .floating-toolbar {
      /* position: absolute;
      bottom: 0;
      left: 0; */
      background-color: #f0f0f0;
      border: 1px solid #ccc;
      border-radius: 4px;
      padding: 4px;
      display: flex;
      flex-wrap: wrap;
      flex-direction: row;
      min-height: 40px;
      gap: 4px;
      z-index: 1000;
      background-color: red;
    }

    .floating-toolbar button {
      background-color: white;
      border: none;
      cursor: pointer;
      font-size: 16px;
      padding: 4px;
      border-radius: 4px;
      aspect-ratio: 1/1;
      width: 50px;
    }

    .floating-toolbar button:hover {
      background-color: #e0e0e0;
    }

    .floating-toolbar button[disabled] {
      opacity: 0.5;
      cursor: not-allowed;
    }

    /* .tooltip {
      position: relative;
    }

    .tooltip::after {
      content: attr(data-tooltip);
      position: absolute;
      bottom: 100%;
      left: 50%;
      transform: translateX(-50%);
      background-color: #333;
      color: #fff;
      padding: 4px 8px;
      border-radius: 4px;
      font-size: 12px;
      white-space: nowrap;
      opacity: 0;
      visibility: hidden;
      transition: opacity 0.2s, visibility 0.2s;
    }

    .tooltip:hover::after {
      opacity: 1;
      visibility: visible;
    } */
  `;

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

    const historyState = createEmptyHistoryState();

    mergeRegister(
      registerRichText(this._editor),
      registerHistory(this._editor, historyState, 300),
    );



    this._editor.registerCommand(
      CAN_UNDO_COMMAND,
      (payload) => {
        this.canUndo = payload;
        return false;
      },
      COMMAND_PRIORITY_LOW
    );

    this._editor.registerCommand(
      CAN_REDO_COMMAND,
      (payload) => {
        this.canRedo = payload;
        return false;
      },
      COMMAND_PRIORITY_LOW
    );

    this._editor.registerCommand(
      UNDO_COMMAND,
      () => {
        this.undo();
        return true;
      },
      COMMAND_PRIORITY_EDITOR
    );

    this._editor.registerCommand(
      REDO_COMMAND,
      () => {
        this.redo();
        return true;
      },
      COMMAND_PRIORITY_EDITOR
    );

    this._editor.registerCommand(
      FORMAT_TEXT_COMMAND,
      (payload) => {
        this._editor.update(() => {
          const selection = $getSelection();
          if ($isRangeSelection(selection)) {
            selection.formatText(payload);
          }
        });
        return true;
      },
      COMMAND_PRIORITY_LOW
    );

    this._editor.registerCommand(
      FORMAT_ELEMENT_COMMAND,
      (payload) => {
        this._editor.update(() => {
          const selection = $getSelection();
          if ($isRangeSelection(selection)) {
            const nodes = selection.getNodes();
            nodes.forEach((node) => {
              if ($isElementNode(node)) {
                node.setFormat(payload);
              }
            });
          }
        });
        return true;
      },
      COMMAND_PRIORITY_LOW
    );




    this._editor.registerCommand(
      UNDO_COMMAND,
      () => {
        this.undo();
        return true;
      },
      COMMAND_PRIORITY_EDITOR
    );

    this._editor.registerCommand(
      REDO_COMMAND,
      () => {
        this.redo();
        return true;
      },
      COMMAND_PRIORITY_EDITOR
    );

    this._editor.registerUpdateListener(() => {
      // this.updateToolbarPosition();
    });
  }

   firstUpdated() {
    this._editor.setRootElement(this.contentEditableRef.value!);

    this._editor.update(() => {
      const root = $getRoot();
      const slotElement = this.shadowRoot?.querySelector('slot');


     this.loadContent().then((loaded) => {
      if(!loaded){
         if (slotElement) {
           const parser = new DOMParser();
           const slotContent = slotElement.assignedNodes()
             .map(node => node.nodeType === Node.TEXT_NODE ? node.textContent : (node as Element).outerHTML)
             .join('');
           const dom = parser.parseFromString(slotContent, 'text/html');
           const nodes = $generateNodesFromDOM(this._editor, dom);
           root.append(...nodes);
         } else {
           // Fallback to $prepopulatedRichText if no slotted content
           $prepopulatedRichText();
         }
      
      }
      
     });

     


    }, { tag: 'history-merge' });

    this.contentEditableRef.value?.addEventListener('blur', () => {
      this.showToolbar = false;
    });

    this.contentEditableRef.value?.addEventListener('focus', () => {
      this.showToolbar = true;
      // this.updateToolbarPosition();
    });

    window.addEventListener('resize', () => {
      if (this.showToolbar) {
        // this.updateToolbarPosition();
      }
    });
  }

  private getSelectedNode(selection: RangeSelection) {
    const anchor = selection.anchor;
    const focus = selection.focus;
    const anchorNode = selection.anchor.getNode();
    const focusNode = selection.focus.getNode();
    if (anchorNode === focusNode) {
      return anchorNode;
    }
    const isBackward = selection.isBackward();
    if (isBackward) {
      return $isAtNodeEnd(focus) ? anchorNode : focusNode;
    } else {
      return $isAtNodeEnd(anchor) ? focusNode : anchorNode;
    }
  }


  private formatText(format: TextFormatType) {
    this._editor.dispatchCommand(FORMAT_TEXT_COMMAND, format);
  }

  private formatElement(format: ElementFormatType) {
    this._editor.dispatchCommand(FORMAT_ELEMENT_COMMAND, format);
  }

  private insertList(type: 'bullet' | 'number') {
    const command = type === 'bullet' ? INSERT_UNORDERED_LIST_COMMAND : INSERT_ORDERED_LIST_COMMAND;
    this._editor.dispatchCommand(command, undefined);
  }

  private removeList() {
    this._editor.dispatchCommand(REMOVE_LIST_COMMAND, undefined);
  }

  private indentContent() {
    // this._editor.dispatchCommand(INDENT_CONTENT_COMMAND, null);
  }

  private outdentContent() {
    // this._editor.dispatchCommand(OUTDENT_CONTENT_COMMAND, null);
  }

  private undo() {
    if (this.canUndo) {
      // this._editor.dispatchCommand(UNDO_COMMAND, null);
    }
  }

  private redo() {
    if (this.canRedo) {
      // this._editor.dispatchCommand(REDO_COMMAND, null);
    }
  }

  private updateToolbarPosition() {
    const selection = window.getSelection();
    if (selection && selection.rangeCount > 0) {
      const range = selection.getRangeAt(0);
      const rect = range.getBoundingClientRect();
      const editorRect = this.contentEditableRef.value?.getBoundingClientRect();
      const toolbarElement = this.shadowRoot?.querySelector('.floating-toolbar') as HTMLElement;

      if (editorRect && toolbarElement) {
        const toolbarRect = toolbarElement.getBoundingClientRect();
        const viewportWidth = window.innerWidth;
        const viewportHeight = window.innerHeight;

        let top = rect.top - editorRect.top - toolbarRect.height - 10; // 10px gap
        let left = rect.left - editorRect.left;

        // Adjust vertical position if toolbar would be outside viewport
        if (top < 0) {
          top = rect.bottom - editorRect.top + 10; // 10px gap
        }

        // Adjust horizontal position if toolbar would be outside viewport
        if (left + toolbarRect.width > viewportWidth) {
          left = viewportWidth - toolbarRect.width - editorRect.left - 10; // 10px from right edge
        }

        // Ensure toolbar doesn't go off the left edge
        left = Math.max(0, left);

        // Adjust for cases where the editor is not at the top of the viewport
        const editorTop = editorRect.top;
        if (top + editorTop < 0) {
          top = Math.max(0, -editorTop + 10); // 10px from top of editor
        } else if (top + editorTop + toolbarRect.height > viewportHeight) {
          top = viewportHeight - editorTop - toolbarRect.height - 10; // 10px from bottom of viewport
        }

        this.toolbarPosition = {
          top: `${top}px`,
          left: `${left}px`,
        };
      }
    }
  }

  async saveContent() {
    const htmlString = await new Promise<string>((resolve) => {
      this._editor.update(() => {
        const htmlString = $generateHtmlFromNodes(this._editor, null);
        resolve(htmlString);
      });
    });

    const result = await RichTextRequest.post(this.id, htmlString);
    if (result.ok) {
      console.log('Content saved successfully. New ID:', result.value);
      // Optionally update the id property if a new one is returned
      this.id = result.value;
    } else {
      console.error('Failed to save content:', result.error);
      // Handle the error (e.g., show an error message to the user)
    }
  }

  async loadContent(): Promise<boolean> {
    if (!this.id) {
      console.warn('No ID provided for loading content');
      return false;
    }

    const result = await RichTextRequest.get(this.id);
    if (result.ok) {
      const htmlContent = result.value; // Assuming this is now an HTML string
      this._editor.update(() => {
        const parser = new DOMParser();
        const dom = parser.parseFromString(htmlContent, 'text/html');
        const nodes = $generateNodesFromDOM(this._editor, dom);
        $getRoot().select();
        $getRoot().clear();
        $getRoot().append(...nodes);
      });
      return true
    } else {
      console.error('Failed to load content:', result.error);
      return false;
      // Handle the error (e.g., show an error message to the user)
    }
  }

  render() {
    return html`
      <div class="editor-wrapper">
        ${this.editable && this.showToolbar ? html`
          <div class="floating-toolbar" style=${styleMap(this.toolbarPosition)}>
            <button @click=${() => this.formatText('bold')} class="tooltip" data-tooltip="Bold"><i class="fas fa-bold"></i></button>
            <button @click=${() => this.formatText('italic')} class="tooltip" data-tooltip="Italic"><i class="fas fa-italic"></i></button>
            <button @click=${() => this.formatText('underline')} class="tooltip" data-tooltip="Underline"><i class="fas fa-underline"></i></button>
            <button @click=${() => this.formatText('strikethrough')} class="tooltip" data-tooltip="Strikethrough"><i class="fas fa-strikethrough"></i></button>
            <button @click=${() => this._editor.dispatchCommand(TOGGLE_LINK_COMMAND, null)} class="tooltip" data-tooltip="Link"><i class="fas fa-link"></i></button>
            <button @click=${() => this.formatElement('left')} class="tooltip" data-tooltip="Align Left"><i class="fas fa-align-left"></i></button>
            <button @click=${() => this.formatElement('center')} class="tooltip" data-tooltip="Align Center"><i class="fas fa-align-center"></i></button>
            <button @click=${() => this.formatElement('right')} class="tooltip" data-tooltip="Align Right"><i class="fas fa-align-right"></i></button>
            <button @click=${() => this.formatElement('justify')} class="tooltip" data-tooltip="Justify"><i class="fas fa-align-justify"></i></button>
            <button @click=${() => this.insertList('bullet')} class="tooltip" data-tooltip="Bullet List"><i class="fas fa-list-ul"></i></button>
            <button @click=${() => this.insertList('number')} class="tooltip" data-tooltip="Numbered List"><i class="fas fa-list-ol"></i></button>
            <button @click=${() => this.removeList()} class="tooltip" data-tooltip="Clear List"><i class="fas fa-list"></i></button>
            <button @click=${() => this.indentContent()} class="tooltip" data-tooltip="Indent"><i class="fas fa-indent"></i></button>
            <button @click=${() => this.outdentContent()} class="tooltip" data-tooltip="Outdent"><i class="fas fa-outdent"></i></button>
            <button @click=${() => this.undo()} ?disabled=${!this.canUndo} class="tooltip" data-tooltip="Undo"><i class="fas fa-undo"></i></button>
            <button @click=${() => this.redo()} ?disabled=${!this.canRedo} class="tooltip" data-tooltip="Redo"><i class="fas fa-redo"></i></button>
          </div>
        ` : ''}
        <span 
          ${ref(this.contentEditableRef)}        
          contenteditable=${this.editable}
          role="textbox" aria-multiline="true"
        ></span>
      </div>
      <slot style="display: none;"></slot>
    `;
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    // window.removeEventListener('resize', this.updateToolbarPosition);
  }


}






