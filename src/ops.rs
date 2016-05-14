use super::*;
use std::io;
use std::mem;
use libc;

pub type NativeHandleType = i32;
pub type NativeSockAddrType = libc::sockaddr;
pub type NativeSockLenType = libc::socklen_t;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum FamilyType {
    Inet = libc::AF_INET as isize,
    Inet6 = libc::AF_INET6 as isize,
    Local = libc::AF_UNIX as isize,
    Packet = libc::AF_PACKET as isize,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum SocketType {
    Stream = libc::SOCK_STREAM as isize,
    Dgram = libc::SOCK_DGRAM as isize,
    Raw = libc::SOCK_RAW as isize,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ProtocolType {
    Default = 0,
    Tcp = libc::IPPROTO_TCP as isize,
    Udp = 17,//libc::IPPROTO_UDP as isize,
    Icmp = 1,//libc::IPPROTO_ICMP as isize,
    IcmpV6 = 58,//libc::IPPROTO_ICMPV6 as isize,
}

pub fn close<'a, S: Socket<'a>>(soc: &mut S) -> io::Result<()> {
    libc_try!(libc::close(*soc.native_handle()));
    Ok(())
}

pub fn shutdown<'a, S: Socket<'a>>(soc: &mut S, how: Shutdown) -> io::Result<()> {
    let how = match how {
        Shutdown::Read => libc::SHUT_RD,
        Shutdown::Write => libc::SHUT_WR,
        Shutdown::Both => libc::SHUT_RDWR,
    };
    libc_try!(libc::shutdown(*soc.native_handle(), how));
    Ok(())
}

pub fn socket<P: Protocol, E: Endpoint<P>>(ep: &E) -> io::Result<i32> {
    Ok(libc_try!(libc::socket(ep.protocol().family_type(ep) as i32, ep.protocol().socket_type(ep) as i32 | libc::SOCK_CLOEXEC, ep.protocol().protocol_type(ep) as i32)))
}

pub fn bind<'a, P: Protocol, S: Socket<'a>, E: Endpoint<P>>(soc: &mut S, ep: &E) -> io::Result<()> {
    libc_try!(libc::bind(*soc.native_handle(), ep.as_sockaddr(), ep.socklen()));
    Ok(())
}

pub fn connect<'a, P: Protocol, S: StreamSocket<'a>, E: Endpoint<P>>(soc: &mut S, ep: &E) -> io::Result<()> {
    libc_try!(libc::connect(*soc.native_handle(), ep.as_sockaddr(), ep.socklen()));
    Ok(())
}

const SOMAXCONN: i32 = 126;
pub fn listen<'a, S: ListenerSocket<'a>>(soc: &mut S) -> io::Result<()> {
    libc_try!(libc::listen(*soc.native_handle(), SOMAXCONN));
    Ok(())
}

pub fn accept<'a, P: Protocol, S: ListenerSocket<'a>, E: Endpoint<P>>(soc: &mut S, ep: &mut E) -> io::Result<i32> {
    let mut socklen = ep.socklen();
    Ok(libc_try!(libc::accept(*soc.native_handle(), ep.as_mut_sockaddr(), &mut socklen)))
}

pub fn get_status_flags<'a, S: Socket<'a>>(soc: &mut S) -> io::Result<i32> {
    Ok(libc_try!(libc::fcntl(*soc.native_handle(), libc::F_GETFL)))
}

pub fn set_status_flags<'a, S: Socket<'a>>(soc: &mut S, flags: i32) -> io::Result<()> {
    libc_try!(libc::fcntl(*soc.native_handle(), libc::F_SETFL, flags));
    Ok(())
}

pub fn get_nonblocking<'a, S: Socket<'a>>(soc: &mut S) -> io::Result<bool> {
    Ok((try!(get_status_flags(soc)) & libc::O_NONBLOCK) != 0)
}

pub fn set_nonblocking<'a, S: Socket<'a>>(soc: &mut S, on: bool) -> io::Result<()> {
    let flags = try!(get_status_flags(soc));
    try!(set_status_flags(soc, if on { flags | libc::O_NONBLOCK } else { flags & !libc::O_NONBLOCK }));
    Ok(())
}

pub fn io_control<'a, S: Socket<'a>, T: IoControlCommand>(soc: &mut S, cmd: &mut T) -> io::Result<()> {
    libc_try!(libc::ioctl(*soc.native_handle(), cmd.name() as u64, cmd.data()));
    Ok(())
}

pub fn get_option<'a, S: Socket<'a>, T: GetOptionCommand>(soc: &mut S, cmd: &mut T) -> io::Result<()> {
    let mut datalen = 0;
    libc_try!(libc::getsockopt(*soc.native_handle(), cmd.level(), cmd.name(), mem::transmute(cmd.data_mut()), &mut datalen));
    cmd.resize(datalen as usize);
    Ok(())
}

pub fn set_option<'a, S: Socket<'a>, T: SetOptionCommand>(soc: &mut S, cmd: &T) -> io::Result<()> {
    libc_try!(libc::setsockopt(*soc.native_handle(), cmd.level(), cmd.name(), mem::transmute(cmd.data()), cmd.size() as libc::socklen_t));
    Ok(())
}

pub fn local_endpoint<'a, P: Protocol, S: Socket<'a>, E: Endpoint<P>>(soc: &mut S, ep: &mut E) -> io::Result<()> {
    let mut socklen = ep.socklen();
    libc_try!(libc::getsockname(*soc.native_handle(), ep.as_mut_sockaddr(), &mut socklen));
    Ok(())
}

pub fn remote_endpoint<'a, P: Protocol, S: Socket<'a>, E: Endpoint<P>>(soc: &mut S, ep: &mut E) -> io::Result<()> {
    let mut socklen = ep.socklen();
    libc_try!(libc::getpeername(*soc.native_handle(), ep.as_mut_sockaddr(), &mut socklen));
    Ok(())
}

pub fn receive<'a, S: Socket<'a>, B: MutableBuffer>(soc: &mut S, mut buf: B) -> io::Result<usize> {
    Ok((libc_try!(libc::recv(*soc.native_handle(), mem::transmute(&mut buf.as_mut_buffer()), buf.buffer_size(), 0))) as usize)
}

pub fn receive_from<'a, P: Protocol, S: Socket<'a>, B: MutableBuffer, E: Endpoint<P>>(soc: &mut S, mut buf: B, ep: &mut E) -> io::Result<usize> {
    let mut socklen = ep.socklen();
    Ok((libc_try!(libc::recvfrom(*soc.native_handle(), mem::transmute(&mut buf.as_mut_buffer()), buf.buffer_size(), 0, ep.as_mut_sockaddr(), &mut socklen))) as usize)
}

pub fn send<'a, S: Socket<'a>, B: Buffer>(soc: &mut S, buf: B) -> io::Result<usize> {
    Ok((libc_try!(libc::send(*soc.native_handle(), mem::transmute(&mut buf.as_buffer()), buf.buffer_size(), 0))) as usize)
}

pub fn send_to<'a, P: Protocol, S: Socket<'a>, B: Buffer, E: Endpoint<P>>(soc: &mut S, buf: B, ep: &E) -> io::Result<usize> {
    Ok((libc_try!(libc::sendto(*soc.native_handle(), mem::transmute(&mut buf.as_buffer()), buf.buffer_size(), 0, ep.as_sockaddr(), ep.socklen()))) as usize)
}