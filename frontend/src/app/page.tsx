'use client'

import { useState, useEffect } from 'react'
import { Menu, X, Users, Trophy, Vote, User } from 'lucide-react'
import { useSidebarStore } from '@/store/sidebarStore'
import { cn } from '@/lib/utils'
import { mockGuilds, mockBounties } from '@/lib/mocks'

export default function HomePage() {
  return (
    <div className="flex flex-col min-h-screen bg-stellar-navy text-stellar-white">
      <header className="p-6 border-b border-stellar-lightNavy">
        <h1 className="text-2xl font-bold">Stellar Guilds</h1>
      </header>
      
      <main className="flex-1 p-6">
        <div className="max-w-4xl mx-auto">
          <div className="bg-stellar-lightNavy rounded-xl p-8 border border-stellar-lightNavy">
            <h1 className="text-3xl font-bold mb-4">Welcome to Stellar Guilds</h1>
            <p className="text-stellar-slate mb-6">
              Your decentralized guild platform is ready! This foundation includes:
            </p>
            <ul className="space-y-2 text-stellar-slate">
              <li>• Next.js 14 with TypeScript</li>
              <li>• Tailwind CSS with custom theme</li>
              <li>• Responsive component library</li>
              <li>• State management with Zustand</li>
              <li>• Mock data system</li>
            </ul>
          </div>
        </div>
      </main>
    </div>
  )
}

function NavItem({ 
  icon, 
  label, 
  href, 
  isActive = false 
}: { 
  icon: React.ReactNode; 
  label: string; 
  href: string; 
  isActive?: boolean; 
}) {
  return (
    <a
      href={href}
      className={cn(
        "flex items-center space-x-3 px-3 py-2 rounded-lg transition-colors",
        isActive 
          ? "bg-stellar-lightNavy text-gold-400 border-l-4 border-gold-500" 
          : "text-stellar-slate hover:bg-stellar-lightNavy hover:text-stellar-lightSlate"
      )}
    >
      {icon}
      <span className="font-medium">{label}</span>
    </a>
  )
}