module.exports = {
  darkMode: 'class',
  content: [
    './index.html',
    './src/**/*.{rs,html,css}',
    './static/**/*.{html,css}',
    '../crates/tailyew/src/**/*.rs',
    '../crates/tailyew/tailyew-safelist.html',
  ],
  theme: {
    extend: {
      colors: {
        'accent-dark': '#E64A19',
        'accent': '#FF5722',
        'background-dark': '#121212',
        'background': '#F7F7F7',
        'border-dark': '#4B5563',
        'border': '#D1D5DB',
        'content-invert': '#F9FAFB',
        'content-muted-dark': '#9CA3AF',
        'content-muted': '#4B5563',
        'content': '#1F2937',
        'danger-dark': '#CC0000',
        'danger': '#FF4C4C',
        'foreground-dark': '#E5E7EB',
        'foreground': '#1F2937',
        'green-500': '#10B981',
        'green-600': '#059669',
        'neutral-dark': '#222222',
        'neutral': '#333333',
        'primary-dark': '#388E3C',
        'primary': '#4CAF50',
        'red-400': '#FCA5A5',
        'red-500': '#F87171',
        'secondary-dark': '#0000CD',
        'secondary': '#0000FF',
        'surface-dark': '#111827',
        'surface-muted': '#F3F4F6',
        'surface': '#FFFFFF',
        'success-dark': '#007E33',
        'success': '#00C851',
      },
      fontFamily: {
        sans: ['Arial', 'Helvetica', 'sans-serif'],
        heading: ['Georgia', 'serif'],
        mono: ['Menlo', 'Monaco', 'monospace'],
      },
      boxShadow: {
        'sm': '0 1px 2px rgba(0, 0, 0, 0.05)',
        'md': '0 4px 6px rgba(0, 0, 0, 0.1)',
        'lg': '0 10px 15px rgba(0, 0, 0, 0.15)',
        'xl': '0 20px 25px rgba(0, 0, 0, 0.2)',
        '2xl': '0 25px 50px rgba(0, 0, 0, 0.25)',
        'inner': 'inset 0 2px 4px rgba(0, 0, 0, 0.06)',
      },
      spacing: {
        '4': '1rem',
        '8': '2rem',
        '16': '4rem',
        '32': '8rem',
        '64': '16rem',
      },
      borderRadius: {
        'sm': '0.125rem',
        'md': '0.375rem',
        'lg': '0.5rem',
        'full': '9999px',
      },
      transitionTimingFunction: {
        'in-out-quint': 'cubic-bezier(0.83, 0, 0.17, 1)',
      },
    },
  },
  safelist: [
    'text-content',
    'dark:text-content-invert',
    'text-content-muted',
    'dark:text-content-muted-dark',
    'text-danger',
    'dark:text-danger-dark',
    'border-border',
    'dark:border-border-dark',
    'bg-surface',
    'dark:bg-surface-dark',
    'opacity-80',
    'opacity-90',
    // Pattern-based safelisting for live theme overrides in demos.
    // This keeps user-entered override classes (bg/text/border/ring/shadow/etc.) from being purged.
    {
      pattern:
        /^(bg|text|border|ring|decoration|shadow)-(slate|gray|zinc|neutral|stone|red|yellow|lime|green|emerald|teal|cyan|blue|violet|fuchsia)-(50|100|200|300|400|500|600|700|800|900|950)(\/(10|20|30|40|50|60|70|80|90))?$/,
      variants: ['hover', 'focus'],
    },
    {
      pattern:
        /^(bg|text|border|ring|decoration)-(white|black|transparent|current)(\/(5|10|20|25|30|40|50|60|70|75|80|90|95))?$/,
      variants: ['hover', 'focus'],
    },
    {
      pattern: /^(font|tracking)-(thin|extralight|light|normal|medium|semibold|bold|extrabold|black|tighter|tight|normal|wide|wider|widest)$/,
    },
    {
      pattern: /^(uppercase|lowercase|capitalize|normal-case|underline|no-underline)$/,
    },
    {
      pattern: /^border-(0|2|4|8)$/,
    },
    {
      pattern: /^(ring-offset|ring)-(0|1|2|4|8)$/,
    },
    {
      pattern: /^shadow(-(sm|md|lg|xl|2xl|inner|none))?$/,
    },
    {
      pattern: /^rounded(-(none|sm|md|lg|xl|2xl|3xl|full))?$/,
    },
    // Existing broad utility patterns
    {
      pattern: /^(top|bottom|left|right)-[0-9]+$/,
    },
    {
      pattern: /^(translate|scale|rotate)-(x|y)?-?[\d/]+$/,
    },
    {
      pattern: /^w-(\d+|full|min|max)/,
    },
    {
      pattern: /^h-(\d+|full|min|max|screen)/,
    },
    {
      pattern: /^rounded(-(sm|md|lg|full))?$/,
    },
    {
      pattern: /^text-(xs|sm|base|lg|xl|2xl|3xl|4xl)$/,
    },
    {
      pattern: /^p[trblxy]?-[0-9]+$/,
    },
    {
      pattern: /^m[trblxy]?-?[0-9]+$/,
    },
  ],
  plugins: [],
};
