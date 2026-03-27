# H1 Heading
## H2 Heading
### H3 Heading
#### H4 Heading
##### H5 Heading
###### H6 Heading

This demo exercises the markdown elements TailYew's `Markdown` component currently renders.

## Supported Elements Summary
| Element | Example | Notes |
| --- | --- | --- |
| Paragraph text | Plain copy in a paragraph | Rendered with TailYew `Typo` |
| Emphasis and strong | *Italic* and **bold** | Inline styles work inside paragraphs, lists, blockquotes, and tables |
| Inline code and links | `cargo check`, [TailYew](https://tailyew.com) | Rendered inline with surrounding text |
| Blockquotes | `> quoted content` | Supports nested inline content and lists |
| Ordered and unordered lists | `- item`, `1. item` | Nested lists work |
| Code blocks | Fenced and indented blocks | `form` fences render dynamic forms |
| Images | `![Alt text](https://...)` | Uses the TailYew `Image` component |
| Tables | Pipe table syntax | Uses the TailYew `Table` component |

## Paragraphs and Inline Formatting
This paragraph combines **strong emphasis**, *italic emphasis*, `inline code`, and a [link to OpenAI](https://openai.com).

This paragraph shows a soft
line break created by a newline in the source.

This sentence ends with a hard break.  
This sentence starts on the next line after the hard break.

## Blockquote
> This is a blockquote with **bold**, *italic*, `inline code`, and a [link](https://tailyew.com).
>
> It also supports nested lists:
> - Quoted unordered item
> - Another quoted item
>
> 1. Quoted ordered item
> 2. Another ordered item

## Unordered List
- Unordered item 1
- Unordered item 2 with **bold**
- Unordered item 3 with a [link](https://yew.rs)
- Nested unordered list
  - Nested item 1
  - Nested item 2 with `inline code`

## Ordered List
1. Ordered item 1
2. Ordered item 2 with *italic*
3. Nested ordered list
   1. Nested ordered item 1
   2. Nested ordered item 2 with **bold**

## Fenced Code Blocks
```rust
fn main() {
    println!("Hello, TailYew!");
}
```

```javascript
const framework = "TailYew";
console.log(`Rendering markdown with ${framework}`);
```

## Indented Code Block
    let message = "Indented code blocks work too.";
    println!("{}", message);

## Table
| Feature | Example | Notes |
| --- | --- | --- |
| Plain text | Works in normal cells | Uses the TailYew `Table` component |
| Emphasis | *Italic* and **bold** | Inline markdown is preserved |
| Inline code | `cargo clippy` | Code spans render inside cells |
| Links | [TailYew Docs](https://tailyew.com) | Links work in table content too |

## Images
![Yew Logo](https://yew.rs/img/logo.png)

This paragraph includes an inline image ![Inline image](https://yew.rs/img/logo.png) alongside text and a [link to GitHub](https://github.com).

## Mixed Content
> **Bold inside Blockquote** with [Link](https://openai.com)

1. Ordered list with `inline code`
2. Second ordered item
   - Nested unordered **bold**
   - Nested unordered with *italic*

```bash
echo "Markdown can mix lists, quotes, links, and code blocks."
```

Final paragraph with multiple styles, `inline code`, and a final [link](https://github.com).

## Dynamic Form Block
```form
{
  "button_label": "Submit Demo Form",
  "inputs": [
    {
      "input": {
        "id": "email",
        "label": "Email",
        "input_type": "Email",
        "placeholder": "Enter your email",
        "required": true
      }
    },
    {
      "input": {
        "id": "name",
        "label": "Name",
        "input_type": "Text",
        "placeholder": "Your Name",
        "required": true
      }
    },
    {
      "checkbox": {
        "id": "accept",
        "label": "Accept Terms",
        "required": true
      }
    }
  ]
}
```
