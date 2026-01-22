import React from 'react'
import { cn } from '@/lib/utils'
import { ModalProps } from '@/types/ui'
import { X } from 'lucide-react'

const Modal = ({ isOpen, onClose, title, children, size = 'md' }: ModalProps) => {
  if (!isOpen) return null

  const sizeClasses = {
    sm: 'max-w-md',
    md: 'max-w-lg',
    lg: 'max-w-2xl',
    xl: 'max-w-4xl',
  }

  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      <div className="flex min-h-screen items-center justify-center p-4">
        <div 
          className="fixed inset-0 bg-black/50 transition-opacity"
          onClick={onClose}
        />
        
        <div 
          className={cn(
            "relative w-full rounded-xl bg-stellar-lightNavy border border-stellar-lightNavy shadow-xl transform transition-all",
            sizeClasses[size]
          )}
        >
          <div className="flex items-center justify-between p-6 border-b border-stellar-lightNavy">
            <h3 className="text-xl font-semibold text-stellar-white">
              {title}
            </h3>
            <button
              onClick={onClose}
              className="p-2 rounded-md hover:bg-stellar-navy transition-colors"
              aria-label="Close modal"
            >
              <X size={20} className="text-stellar-slate" />
            </button>
          </div>
          
          <div className="p-6">
            {children}
          </div>
        </div>
      </div>
    </div>
  )
}

export { Modal }