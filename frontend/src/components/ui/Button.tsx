import React from 'react'
import { cn } from '@/lib/utils'
import { ButtonProps } from '@/types/ui'

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ 
    className, 
    variant = 'primary', 
    size = 'md', 
    isLoading, 
    leftIcon, 
    rightIcon, 
    children, 
    ...props 
  }, ref) => {
    const baseClasses = "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-stellar-slate focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none"
    
    const variantClasses = {
      primary: "bg-gold-500 text-stellar-navy hover:bg-gold-600",
      outline: "border border-stellar-slate bg-transparent hover:bg-stellar-lightNavy hover:text-stellar-white",
      ghost: "hover:bg-stellar-lightNavy hover:text-stellar-white",
      danger: "bg-red-500 text-white hover:bg-red-600",
    }
    
    const sizeClasses = {
      sm: "h-9 px-3 rounded-md text-xs",
      md: "h-10 px-4 py-2",
      lg: "h-11 px-8 rounded-md text-base",
    }
    
    return (
      <button
        className={cn(
          baseClasses,
          variantClasses[variant],
          sizeClasses[size],
          className
        )}
        ref={ref}
        disabled={isLoading || props.disabled}
        {...props}
      >
        {isLoading ? (
          <>
            <span className="animate-spin mr-2">●</span>
            Loading...
          </>
        ) : (
          <>
            {leftIcon && <span className="mr-2">{leftIcon}</span>}
            {children}
            {rightIcon && <span className="ml-2">{rightIcon}</span>}
          </>
        )}
      </button>
    )
  }
)

Button.displayName = "Button"

export { Button }