'use client'

import React from 'react'
import { Menu, X, Users, Trophy, Vote, User } from 'lucide-react'
import { useSidebarStore } from '@/store/sidebarStore'
import { cn } from '@/lib/utils'

interface HeaderProps {
  className?: string
}

const Header = ({ className }: HeaderProps) => {
  const { isOpen, toggleSidebar } = useSidebarStore()

  return (
    <header className={cn(
      "sticky top-0 z-50 w-full border-b border-stellar-lightNavy bg-stellar-darkNavy/80 backdrop-blur",
      className
    )}>
      <div className="container flex h-16 items-center px-4">
        <button
          onClick={toggleSidebar}
          className="mr-4 p-2 rounded-md hover:bg-stellar-lightNavy transition-colors md:hidden"
          aria-label="Toggle sidebar"
        >
          {isOpen ? <X size={20} /> : <Menu size={20} />}
        </button>
        
        <div className="flex items-center space-x-2">
          <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-gold-500 to-gold-700 flex items-center justify-center">
            <span className="text-stellar-navy font-bold text-sm">SG</span>
          </div>
          <h1 className="text-xl font-bold text-stellar-white">Stellar Guilds</h1>
        </div>

        <nav className="hidden md:flex items-center space-x-6 ml-auto">
          <a href="#" className="text-stellar-slate hover:text-stellar-lightSlate transition-colors">Guilds</a>
          <a href="#" className="text-stellar-slate hover:text-stellar-lightSlate transition-colors">Bounties</a>
          <a href="#" className="text-stellar-slate hover:text-stellar-lightSlate transition-colors">Governance</a>
        </nav>

        <div className="ml-4 flex items-center space-x-3">
          <button className="p-2 rounded-full hover:bg-stellar-lightNavy transition-colors">
            <Vote size={20} className="text-stellar-slate" />
          </button>
          <button className="p-2 rounded-full hover:bg-stellar-lightNavy transition-colors">
            <User size={20} className="text-stellar-slate" />
          </button>
        </div>
      </div>
    </header>
  )
}

export { Header }