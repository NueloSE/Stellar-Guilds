import React from 'react'
import { cn } from '@/lib/utils'
import { CardProps } from '@/types/ui'

const Card = React.forwardRef<HTMLDivElement, CardProps>(
  ({ className, title, description, footer, children, ...props }, ref) => {
    return (
      <div
        className={cn(
          "rounded-xl border border-stellar-lightNavy bg-stellar-lightNavy shadow-card hover:shadow-card-hover transition-shadow",
          className
        )}
        ref={ref}
        {...props}
      >
        {(title || description) && (
          <div className="p-6 pb-4">
            {title && (
              <h3 className="text-xl font-semibold text-stellar-white mb-2">
                {title}
              </h3>
            )}
            {description && (
              <p className="text-stellar-slate">
                {description}
              </p>
            )}
          </div>
        )}
        {children && (
          <div className={cn("p-6", (title || description) && "pt-0")}>
            {children}
          </div>
        )}
        {footer && (
          <div className="p-6 pt-0 border-t border-stellar-lightNavy">
            {footer}
          </div>
        )}
      </div>
    )
  }
)

Card.displayName = "Card"

export { Card }