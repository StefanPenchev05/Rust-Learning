import React, { ReactNode } from 'react'
import NavBar from '../NavBar'

interface LayoutProps {
    children: ReactNode
}

const Layout:React.FC<LayoutProps> = ({children}) => {
  return (
    <div className=''>
        <header>
            <NavBar/>
        </header>
        <main>
            {children}
        </main>

        <footer>

        </footer>
    </div>
  )
}

export default Layout