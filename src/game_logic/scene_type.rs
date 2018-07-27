// mehens_portable_casino. A gambling game made using ggez and Dicecoin
// Copyright (C) 2018  Ian L. Gore
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/*
These are the various types of scenes we can implement. The only special purpose enum here is
Exit, which will cause the handler to immediately exit cleanly.

*/
//This allows us to format the SceneType with {:?} in println!(...)
#[derive(Debug, Copy, Clone)]
pub enum SceneType {
    Cutscene,
    Game,
    Menu,
    Pause,
    Credits,
    Exit,
}

