import React from 'react'
import { cn } from '@/lib/utils'

interface FooterProps {
  className?: string
}

const Footer = ({ className }: FooterProps) => {
  return (
    <footer className={cn(
      "border-t border-stellar-lightNavy bg-stellar-darkNavy py-6 mt-auto",
      className
    )}>
      <div className="container px-4">
        <div className="flex flex-col md:flex-row justify-between items-center">
          <div className="text-stellar-slate text-sm">
            © 2024 Stellar Guilds. Built on the Stellar Network.
          </div>
          <div className="flex space-x-6 mt-4 md:mt-0">
            <a href="#" className="text-stellar-slate hover:text-stellar-lightSlate transition-colors text-sm">
              Documentation
            </a>
            <a href="#" className="text-stellar-slate hover:text-stellar-lightSlate transition-colors text-sm">
              GitHub
            </a>
            <a href="#" className="text-stellar-slate hover:text-stellar-lightSlate transition-colors text-sm">
              Discord
            </a>
          </div>
        </div>
      </div>
    </footer>
  )
}

export { Footer }