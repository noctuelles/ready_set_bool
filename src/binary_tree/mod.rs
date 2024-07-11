/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mod.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/11 17:08:41 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/11 17:31:51 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug)]
pub struct BTreeNode<T> {
    pub value: T,
    pub right: Option<Box<BTreeNode<T>>>,
    pub left: Option<Box<BTreeNode<T>>>,
}
