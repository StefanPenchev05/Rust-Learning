import React, { ReactNode } from 'react'
import NavBar from '../NavBar'

interface Layout{
    children: ReactNode
}

const Layout:React.FC<Layout> = ({children}) => {
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