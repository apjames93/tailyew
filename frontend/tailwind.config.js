module.exports = {
  darkMode: 'class',
  content: [
    "../crates/tailyew/src/**/*.rs",
    './**/**/*.{html,js,rs}',
  ],
  theme: {
    extend: {
      colors: {
        'accent-dark': '#E64A19',
        'accent': '#FF5722',
        'background-dark': '#121212',
        'background': '#F7F7F7',
        'danger-dark': '#CC0000',
        'danger': '#FF4C4C',
        'foreground-dark': '#1E1E1E',
        'foreground': '#FFFFFF',
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
    // Position, z-index, overlay
    'fixed', 'absolute', 'relative', 'static', 'inset-0', 'inset-y-0', 'top-0', 'bottom-0', 'left-0', 'right-0', 'top-1', 'top-4', 'top-5', 'top-full', 'bottom-full', 'left-1/2', 'left-full', 'right-full', 'z-30', 'z-40', 'z-50', 'z-[-1]',
    // Backgrounds & opacity
    'bg-black', 'bg-opacity-40', 'bg-white', 'dark:bg-gray-900', 'bg-danger', 'bg-neutral', 'bg-success', 'bg-transparent', 'dark:bg-accent-dark', 'dark:bg-danger-dark', 'dark:bg-primary-dark', 'dark:bg-neutral-dark', 'dark:bg-red-800', 'dark:bg-secondary-dark', 'dark:bg-success-dark', 'dark:bg-yellow-700',
    // Borders, rings, shadows
    'border', 'border-primary', 'border-accent', 'border-transparent', 'border-gray-200', 'border-gray-300', 'border-gray-400', 'border-blue-400', 'border-blue-500', 'border-green-400', 'border-red-400', 'border-red-500', 'border-yellow-400', 'border-t-blue-500', 'dark:border-primary-dark', 'dark:border-blue-300', 'dark:border-gray-600', 'dark:border-gray-700',
    'ring', 'ring-1', 'ring-2', 'ring-primary', 'ring-accent', 'ring-blue-400', 'ring-green-500', 'ring-red-500', 'ring-offset-1', 'ring-offset-2', 'dark:ring-accent-dark', 'dark:ring-blue-600', 'dark:ring-green-400', 'dark:ring-primary-dark',
    'shadow', 'shadow-sm', 'shadow-md', 'shadow-lg', 'shadow-xl', 'hover:shadow-2xl',
    // Typography & text
    'prose', 'dark:prose-invert', 'text-left', 'max-w-none', 'prose-sm', 'prose-lg', 'prose-xl', 'font-light', 'font-normal', 'font-medium', 'font-semibold', 'font-bold', 'italic', 'border-l-4', 'pl-4', 'my-4', 'text-xs', 'text-sm', 'text-base', 'text-lg', 'text-xl', 'text-2xl', 'text-3xl', 'text-4xl', 'mb-1', 'mb-2', 'mb-3', 'mb-4', 'mb-5', 'mb-6', 'mb-8', 'mt-1', 'mt-2', 'mt-4', 'mt-6', 'mt-8',
    'text-primary', 'text-red-500', 'text-red-600', 'text-red-800', 'text-yellow-800', 'text-gray-200', 'text-gray-500', 'text-gray-600', 'text-gray-700', 'text-gray-800', 'text-gray-900', 'text-blue-400', 'text-blue-500', 'dark:text-primary-dark', 'dark:text-red-200', 'dark:text-red-400', 'dark:text-yellow-200', 'dark:text-gray-100', 'dark:text-gray-200', 'dark:text-gray-300', 'dark:text-gray-400', 'dark:text-blue-200', 'dark:text-blue-300',
    // Spacing, radius, sizing
    'p-1', 'p-2', 'p-3', 'p-4', 'p-6', 'p-8', 'px-1', 'px-2', 'px-3', 'px-4', 'py-1', 'py-2', 'py-3', 'py-4', 'py-12', 'm-0', 'm-1', 'm-2', 'm-3', 'm-4', 'mx-auto', 'my-4', 'ml-2', 'ml-6', 'ml-auto', 'mr-2', 'rounded', 'rounded-md', 'rounded-lg', 'rounded-full',
    'container', 'max-w-7xl', 'max-w-md', 'max-w-sm', 'max-w-none', 'max-w-full', 'min-w-full', 'min-w-[120px]', 'w-1/2', 'w-1/4', 'w-3/4', 'w-4', 'w-8', 'w-10', 'w-14', 'w-16', 'w-64', 'h-1', 'h-2', 'h-4', 'h-8', 'h-10', 'h-12', 'h-16', 'h-48', 'h-auto', 'h-full', 'h-screen', 'max-h-0', 'max-h-[400px]', 'max-h-[80vh]', 'min-h-screen',
    // Flex, grid, alignment
    'flex', 'flex-row', 'flex-col', 'flex-wrap', 'items-start', 'items-center', 'items-end', 'justify-start', 'justify-center', 'justify-between', 'justify-end', 'gap-1', 'gap-2', 'gap-3', 'gap-4', 'gap-6', 'gap-8', 'grid', 'grid-cols-1', 'grid-cols-2', 'grid-cols-3',
    // Animation, cursor, visibility
    'transition-all', 'transition-colors', 'transition-transform', 'transition-[max-height]', 'duration-150', 'duration-200', 'duration-300', 'duration-500', 'ease-in-out', 'transform', 'origin-top', 'opacity-0', 'opacity-100', 'scale-y-0', 'scale-y-100', 'pointer-events-none', 'pointer-events-auto', 'hidden', 'block', 'inline-block', 'animate-spin', 'animate-pulse', 'cursor-pointer', 'cursor-default', 'cursor-not-allowed', 'visible', 'invisible',
    // Overflow, scrollbar
    'overflow-auto', 'overflow-hidden', 'overflow-x-auto', 'overflow-y-auto', 'truncate', 'whitespace-nowrap', 'scrollbar-thin', 'scrollbar-thumb-gray-400', 'scrollbar-track-gray-100', 'dark:scrollbar-thumb-gray-700', 'dark:scrollbar-track-gray-900',
    // Object fit, font
    'font-mono', 'object-cover',
    // Focus, ring, outline
    'focus:outline-none', 'focus:ring-2', 'focus:ring-offset-2', 'focus:ring-accent', 'focus:ring-blue-400', 'focus:ring-green-500', 'focus:ring-primary', 'focus:ring-red-500', 'focus:border-green-500', 'focus:border-primary', 'focus:border-red-500', 'dark:focus:border-primary-dark', 'dark:focus:ring-accent-dark', 'dark:focus:ring-blue-600', 'dark:focus:ring-green-400', 'dark:focus:ring-primary-dark',
    // Hover & dark hover
    'hover:bg-primary-dark', 'hover:bg-secondary-dark', 'hover:bg-danger-dark', 'hover:bg-success-dark', 'hover:bg-accent-dark', 'hover:bg-gray-100', 'hover:bg-gray-200', 'hover:bg-gray-300', 'hover:bg-blue-50', 'hover:text-blue-500', 'hover:text-blue-700', 'hover:scale-105',
    'dark:hover:bg-primary', 'dark:hover:bg-secondary', 'dark:hover:bg-danger', 'dark:hover:bg-success', 'dark:hover:bg-accent', 'dark:hover:bg-blue-700', 'dark:hover:bg-gray-700', 'dark:hover:bg-gray-800', 'dark:hover:text-blue-300', 'dark:hover:text-blue-500', 'dark:hover:text-gray-500', 'dark:hover:text-gray-600',
    // Responsive utilities
    'sm:flex', 'sm:grid-cols-2', 'sm:px-6', 'md:flex', 'md:hidden', 'md:w-80', 'md:max-w-md', 'md:grid', 'md:grid-cols-2', 'md:grid-cols-3', 'md:items-start', 'lg:block', 'lg:grid-cols-3', 'lg:px-8', 'lg:w-96', 'xl:inline-block',
    // Input/button/select states
    'w-full', 'checked:bg-primary', 'checked:border-transparent', 'checked:ring-primary', 'appearance-none',
  ],
  plugins: [],
};
