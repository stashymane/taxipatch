use retour::Error;
use std::marker::PhantomData;
use thiserror::Error;
use windows::core::PCSTR;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};

pub type Result<T> = core::result::Result<T, LocatorError>;

#[derive(Debug, Clone, Copy)]
pub enum ModuleLocator {
    Current,
    Named(PCSTR),
    Handle(HMODULE),
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum AddressLocator {
    Offset(usize),
    Export(PCSTR),
}

#[derive(Debug, Clone, Copy)]
pub struct PtrLocator<T> {
    module: ModuleLocator,
    address: AddressLocator,
    _marker: PhantomData<T>,
}

#[derive(Debug, Error)]
pub enum LocatorError {
    #[error("Failed to find module")]
    ModuleNotFound(#[from] windows::core::Error),
    #[error("Failed to find function in module")]
    FunctionNotFound,
    #[error("Failed to initialize hook: {0}")]
    InitFailed(#[from] Error),
}

impl ModuleLocator {
    pub unsafe fn resolve(self) -> Result<HMODULE> {
        match self {
            Self::Current => unsafe {
                GetModuleHandleA(None).map_err(LocatorError::ModuleNotFound)
            },
            Self::Named(name) => unsafe {
                GetModuleHandleA(name).map_err(LocatorError::ModuleNotFound)
            },
            Self::Handle(module) => Ok(module),
            Self::None => Ok(HMODULE::default()),
        }
    }
}

impl AddressLocator {
    pub unsafe fn resolve(self, module: HMODULE) -> Result<usize> {
        match self {
            Self::Offset(rva) => Ok(module.0 as usize + rva),

            Self::Export(name) => unsafe {
                GetProcAddress(module, name)
                    .map(|proc| proc as usize)
                    .ok_or(LocatorError::FunctionNotFound)
            },
        }
    }
}

impl<T> PtrLocator<T> {
    pub const fn new(module: ModuleLocator, address: AddressLocator) -> Self {
        Self {
            module,
            address,
            _marker: PhantomData,
        }
    }

    pub const fn offset(module: ModuleLocator, offset: usize) -> Self {
        Self::new(module, AddressLocator::Offset(offset))
    }

    pub const fn export(module: ModuleLocator, name: PCSTR) -> Self {
        Self::new(module, AddressLocator::Export(name))
    }

    pub const fn absolute(address: usize) -> Self {
        Self::new(ModuleLocator::None, AddressLocator::Offset(address))
    }

    pub fn get_address(&self) -> Result<usize> {
        get_address(self.module, self.address)
    }

    pub fn ptr(&self) -> Result<*mut T> {
        Ok(self.get_address()? as *mut T)
    }

    pub unsafe fn read(&self) -> Result<T>
    where
        T: Copy,
    {
        let ptr = self.ptr()?;
        Ok(unsafe { *ptr })
    }

    pub unsafe fn write(&self, value: T) -> Result<()> {
        let ptr = self.ptr()?;
        unsafe {
            *ptr = value;
        }
        Ok(())
    }
}

fn get_address(module: ModuleLocator, addr: AddressLocator) -> Result<usize> {
    match addr {
        address => {
            let module = unsafe { module.resolve()? };
            unsafe { address.resolve(module) }
        }
    }
}
